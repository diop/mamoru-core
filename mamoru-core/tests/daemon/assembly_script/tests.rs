use expect_test::expect;
use test_log::test;

use mamoru_core::{DataError, IncidentSeverity};
use mamoru_core_test_utils::assembly_script::{AssemblyScriptModule, AS_SDK_PATH};
use mamoru_core_test_utils::test_blockchain_data::data_ctx;
use mamoru_core_test_utils::{test_daemon, test_daemon_with_parameters};

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
              report('txHash', IncidentSeverity.Alert, "Test");
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

    expect![[r#"
        Incident {
            severity: Alert,
            message: "Test",
            tx_hash: "txHash",
            address: "",
            data: [],
        }
    "#]]
    .assert_debug_eq(&result.incidents[0]);
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
              report('txHash', IncidentSeverity.Alert, "Test");
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
                   report('txHash', IncidentSeverity.Alert, "Test");
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
#[ignore]
async fn http() {
    const AS_CODE_BLOCK: &str = r#"""
        import {http, HttpMethod, report, IncidentSeverity} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
           let response = http(HttpMethod.GET, ENDPOINT);

           let _body = response.body();
           let _headers = response.headers();
           let _error = response.error();

           if (response.status() == 418) {
               report('txHash', IncidentSeverity.Alert, "Test");
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
               report('txHash', IncidentSeverity.Alert, "Test");
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
        import {report, IncidentSeverity} from "@mamoru-ai/mamoru-sdk-as/assembly";

        export function main(): void {
            let data = new Uint8Array(4);
            data.set([0, 1, 2, 3])

            report('txHash', IncidentSeverity.Alert, "Test", data, "0x0");
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

    assert_eq!(&incident.data, &[0, 1, 2, 3]);
}

#[test(tokio::test)]
async fn u256_from_str() {
    let ctx = data_ctx("DUMMY_HASH");
    let module = AssemblyScriptModule::with_deps(
        r#"""
        import {report, IncidentSeverity, u256} from "@mamoru-ai/mamoru-sdk-as/assembly";
        import {u256FromStr} from "@mamoru-ai/mamoru-sdk-as/assembly/util";

        export function main(): void {
            const senceOfLife = u256FromStr("42");

            if (senceOfLife.toString() == "42") {
                report('txHash', IncidentSeverity.Alert, "senceOfLife");
            }

            const maxU256Decimal = u256FromStr("115792089237316195423570985008687907853269984665640564039457584007913129639935");

            if (maxU256Decimal.toString() == "115792089237316195423570985008687907853269984665640564039457584007913129639935") {
                report('txHash', IncidentSeverity.Alert, "maxU256Decimal");
            }

            const maxU256 = u256FromStr("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

            if (maxU256.toString() == "115792089237316195423570985008687907853269984665640564039457584007913129639935") {
                report('txHash', IncidentSeverity.Alert, "maxU256");
            }

            const zero = u256FromStr("0");

            if (zero.toString() == "0") {
                report('txHash', IncidentSeverity.Alert, "zero");
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

    assert_eq!(result.incidents.len(), 4);
}
