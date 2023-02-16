use crate::daemon::assembly_script::{active_daemon, AssemblyScriptModule};
use expect_test::expect;
use mamoru_core::test_blockchain_data::data_ctx;
use mamoru_core::DataError;
use test_log::test;

#[test(tokio::test)]
async fn main_function_missing_fails() {
    let ctx = data_ctx("DUMMY_HASH");

    let module = AssemblyScriptModule::new(
        r#"""
        export function daemon(): void {

        }
    """#,
    );

    let daemon = active_daemon(&module);
    let result = daemon.verify(&ctx).await;

    expect![[r#"
        Err(
            WasmExport {
                source: Missing(
                    "main",
                ),
                export: "main",
            },
        )
    "#]]
    .assert_debug_eq(&result);
}

#[test(tokio::test)]
async fn main_function_wrong_signature_fails() {
    let ctx = data_ctx("DUMMY_HASH");

    let module = AssemblyScriptModule::new(
        r#"""
        export function main(data: number): void {

        }
    """#,
    );

    let daemon = active_daemon(&module);
    let result = daemon.verify(&ctx).await;

    expect![[r#"
        Err(
            WasmExport {
                source: IncompatibleType,
                export: "main",
            },
        )
    "#]]
    .assert_debug_eq(&result);
}

#[test(tokio::test)]
async fn main_function_valid_signature_empty_result() {
    let ctx = data_ctx("DUMMY_HASH");

    let module = AssemblyScriptModule::new(
        r#"""
        export function main(): void {}
    """#,
    );

    let daemon = active_daemon(&module);
    let result = daemon.verify(&ctx).await;

    expect![[r#"
        Ok(
            VerifyCtx {
                matched: false,
                incidents: [],
            },
        )
    "#]]
    .assert_debug_eq(&result);
}

#[test(tokio::test)]
async fn generates_many_incidents() {
    let ctx = data_ctx("DUMMY_HASH");

    let module = AssemblyScriptModule::new(
        r#"""
        @external("mamoru", "report")
        declare function report(): void

        export function main(): void {
            for (let i = 0; i < 10; i++) {
              report();
            }
        }
    """#,
    );

    let daemon = active_daemon(&module);
    let result = daemon
        .verify(&ctx)
        .await
        .expect("Failed to run Daemon::verify()");

    assert!(result.matched);
    assert_eq!(result.incidents.len(), 10);
}

#[test(tokio::test)]
async fn too_many_incident_generation_fails() {
    let ctx = data_ctx("DUMMY_HASH");

    let module = AssemblyScriptModule::new(
        r#"""
        @external("mamoru", "report")
        declare function report(): void

        export function main(): void {
            // assuming 1000 is too many
            for (let i = 0; i < 1_000; i++) {
              report();
            }
        }
    """#,
    );

    let daemon = active_daemon(&module);

    if let Err(DataError::WasmRuntime(err)) = daemon.verify(&ctx).await {
        expect!["sending on a full channel"].assert_eq(&err.message());
    } else {
        panic!("Expected error, but no error returned.")
    }
}

#[test(tokio::test)]
async fn invalid_query_fails() {
    let ctx = data_ctx("DUMMY_HASH");

    let module = AssemblyScriptModule::new(
        r#"""
        @external("mamoru", "query")
        declare function query(query: string): string

        export function main(): void {
            query("Definitely not an SQL query =)");
        }
    """#,
    );

    let daemon = active_daemon(&module);

    if let Err(DataError::WasmRuntime(err)) = daemon.verify(&ctx).await {
        expect![
            "Failed to parse SQL: sql parser error: Expected an SQL statement, found: Definitely"
        ]
        .assert_eq(&err.message());
    } else {
        panic!("Expected error, but no error returned.")
    }
}

#[test(tokio::test)]
async fn smoke() {
    let ctx = data_ctx("DUMMY_HASH");
    let module = AssemblyScriptModule::with_deps(
        r#"""
        import { JSON } from "assemblyscript-json/assembly";

        @external("mamoru", "query")
        declare function mamoru_query(query: string): string

        @external("mamoru", "report")
        declare function report(): void

        function query(query: string): Array<JSON.Obj> {
            let query_result: string = mamoru_query(query);
            let json: JSON.Arr = <JSON.Arr>(JSON.parse(query_result));

            return json.valueOf().map((value: JSON.Value) => value as JSON.Obj);
        }

        export function main(): void {
           let rows = query("SELECT t.gas_used FROM transactions t WHERE t.digest = 'DUMMY_HASH'");

           rows.forEach(value => {
               let gas_used = value.getInteger("gas_used")!.valueOf();

               if (gas_used == 42_000) {
                   report();
               }
           });
        }
    """#,
        &["assemblyscript-json@1.1.0"],
    );

    let daemon = active_daemon(&module);

    let result = daemon
        .verify(&ctx)
        .await
        .expect("Failed to run Daemon::verify()");

    assert!(result.matched);
    assert_eq!(result.incidents.len(), 1);
}

#[test(tokio::test)]
async fn http() {
    const AS_CODE_BLOCK: &str = r#"""
        import { JSON, JSONEncoder } from "assemblyscript-json/assembly";

        @external("mamoru", "http")
        declare function mamoru_http(request: string): string

        @external("mamoru", "report")
        declare function report(): void

        function http_get(url: string): i64 {
            let encoder = new JSONEncoder();
            encoder.pushObject(null);
            encoder.setString("method", "GET");
            encoder.setString("url", url);
            encoder.popObject();

            let requestPayload: string = encoder.toString();

            let query_result: string = mamoru_http(requestPayload);
            let json: JSON.Obj = <JSON.Obj>(JSON.parse(query_result));

            return json.getInteger("status")!.valueOf();
        }

        export function main(): void {
           let response_code = http_get(ENDPOINT);

           if (response_code == 418) {
               report();
           }
        }
    """#;

    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/teapot")
        .with_status(418)
        .create_async()
        .await;

    let endpoint = format!("{}{}", server.url(), "/teapot");

    let module = AssemblyScriptModule::with_deps(
        &format!(
            "const ENDPOINT: string = \"{}\";\n{}",
            endpoint, AS_CODE_BLOCK
        ),
        &["assemblyscript-json@1.1.0"],
    );

    let daemon = active_daemon(&module);
    let ctx = data_ctx("DUMMY_HASH");

    let result = daemon
        .verify(&ctx)
        .await
        .expect("Failed to run Daemon::verify()");

    assert!(result.matched);
    assert_eq!(result.incidents.len(), 1);

    // assert endpoint was called
    mock.assert_async().await;
}
