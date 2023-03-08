use crate::daemon::assembly_script::{
    test_daemon, test_daemon_with_parameters, AssemblyScriptModule,
};
use expect_test::expect;
use mamoru_core::{test_blockchain_data::data_ctx, DataError, IncidentSeverity};
use std::collections::BTreeMap;
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

    let daemon = test_daemon(&module);
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

    let daemon = test_daemon(&module);
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

    let daemon = test_daemon(&module);
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
        import {report, IncidentSeverity} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
            for (let i = 0; i < 10; i++) {
              report(IncidentSeverity.Alert, "Test");
            }
        }
    """#,
        &[AS_SDK_PATH],
    );

    let daemon = test_daemon(&module);
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
        import {report, IncidentSeverity} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
            // assuming 1000 is too many
            for (let i = 0; i < 1_000; i++) {
              report(IncidentSeverity.Alert, "Test");
            }
        }
    """#,
        &[AS_SDK_PATH],
    );

    let daemon = test_daemon(&module);

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

    let daemon = test_daemon(&module);

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
        import {query, report, IncidentSeverity} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
           let rows = query("SELECT t.gas_used FROM transactions t WHERE t.digest = 'DUMMY_HASH'");

           rows.forEach(value => {
               let gas_used = value.getInteger("gas_used")!.valueOf();

               if (gas_used == 42_000) {
                   report(IncidentSeverity.Alert, "Test");
               }
           });
        }
    """#,
        &[AS_SDK_PATH],
    );

    let daemon = test_daemon(&module);

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
        import {http, HttpMethod, report, IncidentSeverity} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
           let response = http(HttpMethod.GET, ENDPOINT);

           if (response.status() == 418) {
               report(IncidentSeverity.Alert, "Test");
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

    let daemon = test_daemon(&module);
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

#[test(tokio::test)]
async fn parameter() {
    let ctx = data_ctx("DUMMY_HASH");
    let module = AssemblyScriptModule::with_deps(
        r#"""
        import {parameter, report, IncidentSeverity} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
           let boolParam = parameter("bool");
           let boolValid = boolParam.asBoolean() == true && isNaN(boolParam.asNumber());

           let numberParam = parameter("number");
           let numberValid = Math.round(numberParam.asNumber()) == 42 && numberParam.asBoolean() == false;

           let stringParam = parameter("string");
           let stringValid = stringParam.asString() == "hello";

           if (boolValid && numberValid && stringValid) {
               report(IncidentSeverity.Alert, "Test");
           }
        }
    """#,
        &[AS_SDK_PATH],
    );

    let daemon = test_daemon_with_parameters(
        &module,
        maplit::hashmap! {
            "bool".to_string() => "true".to_string(),
            "number".to_string() => "42.01".to_string(),
            "string".to_string() => "hello".to_string(),
        },
    );

    let result = daemon
        .verify(&ctx)
        .await
        .expect("Failed to run Daemon::verify()");

    assert!(result.matched);
    assert_eq!(result.incidents.len(), 1);
}

#[test(tokio::test)]
async fn incident_report_data_deserialization() {
    let ctx = data_ctx("DUMMY_HASH");
    let module = AssemblyScriptModule::with_deps(
        r#"""
        import {report, IncidentSeverity, IncidentDataStruct, StringDataValue} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
            let dataNested = new IncidentDataStruct();
            dataNested.addString("string", "nested");

            let data = new IncidentDataStruct();

            data.addNull("null");
            data.addNumber("number", 42.0);
            data.addString("string", "hello");
            data.addBoolean("boolean", true);
            data.addList("list", [
                new StringDataValue("first"),
                new StringDataValue("second"),
            ]);
            data.addStruct("struct", dataNested);


            report(IncidentSeverity.Alert, "Test", data, "0x0");
        }
    """#,
        &[AS_SDK_PATH],
    );

    let daemon = test_daemon(&module);

    let mut result = daemon
        .verify(&ctx)
        .await
        .expect("Failed to run Daemon::verify()");

    assert!(result.matched);

    let incident = result.incidents.remove(0);

    assert_eq!(incident.severity, IncidentSeverity::Alert);
    assert_eq!(incident.message, "Test");
    assert_eq!(incident.address, "0x0");

    // important to use `BTreeMap` instead of `HashMap` to have stable key ordering for `assert_debug_eq`
    let data: BTreeMap<_, _> = incident.data.fields().into_iter().collect();

    expect![[r#"
        {
            "boolean": Bool(
                true,
            ),
            "list": List(
                [
                    String(
                        "first",
                    ),
                    String(
                        "second",
                    ),
                ],
            ),
            "null": Null,
            "number": Number(
                42.0,
            ),
            "string": String(
                "hello",
            ),
            "struct": Struct(
                IncidentDataStruct {
                    fields: {
                        "string": String(
                            "nested",
                        ),
                    },
                },
            ),
        }
    "#]]
    .assert_debug_eq(&data);
}
