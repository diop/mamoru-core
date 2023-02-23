use crate::daemon::assembly_script::{active_daemon, AssemblyScriptModule};
use expect_test::expect;
use mamoru_core::test_blockchain_data::data_ctx;
use mamoru_core::DataError;
use test_log::test;

const AS_SDK_PATH: &str = concat!("file:", env!("CARGO_MANIFEST_DIR"), "/../mamoru-sdk-as");

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

    let module = AssemblyScriptModule::with_deps(
        r#"""
        import {report} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
            for (let i = 0; i < 10; i++) {
              report();
            }
        }
    """#,
        &[AS_SDK_PATH],
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

    let module = AssemblyScriptModule::with_deps(
        r#"""
        import {report} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
            // assuming 1000 is too many
            for (let i = 0; i < 1_000; i++) {
              report();
            }
        }
    """#,
        &[AS_SDK_PATH],
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

    let module = AssemblyScriptModule::with_deps(
        r#"""
        import {query} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
            query("Definitely not an SQL query =)");
        }
    """#,
        &[AS_SDK_PATH],
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
        import {query, report} from "@mamoru-ai/mamoru-sdk-as/assembly";

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
        &[AS_SDK_PATH],
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
        import {http, HttpRequest, HttpMethod, report} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
           let response = http(new HttpRequest(HttpMethod.GET, ENDPOINT));

           if (response.status() == 418) {
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
        &[AS_SDK_PATH],
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
