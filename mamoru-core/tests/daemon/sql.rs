use expect_test::expect;
use maplit::hashmap;
use test_log::test;

use mamoru_core::DataError;
use mamoru_core_test_utils::test_blockchain_data::data_ctx;

use crate::daemon::{test_sql_daemon, TestDaemon};

#[test(tokio::test)]
async fn simple_query_matches() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon("SELECT t.seq FROM transactions t WHERE t.digest = 'DUMMY_HASH'");

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn cte_expressions_are_supported() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        WITH func1_traces AS (
            SELECT * FROM call_traces ct WHERE ct.function = 'func1'
        )

        SELECT t.digest FROM transactions t
            INNER JOIN func1_traces ct ON ct.seq IS NOT NULL
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn write_expressions_are_not_allowed() -> Result<(), DataError> {
    let statements = [
        "INSERT INTO call_traces VALUES (3, 4, 'func3')",
        "CREATE EXTERNAL TABLE foo STORED AS CSV LOCATION 'server_passwords.csv'",
    ];

    for statement in statements {
        let daemon = TestDaemon::builder()
            .expression(statement)
            .build()
            .into_daemon();

        assert!(matches!(daemon, Err(DataError::UnsupportedStatement)));
    }

    Ok(())
}

#[test(tokio::test)]
async fn sql_parameter_substitution_works() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");

    let daemon = TestDaemon::builder()
        .expression("SELECT t.digest FROM transactions t WHERE t.digest = '{{ digest }}'")
        .parameters(hashmap! {
            "digest".to_string() => "DUMMY_HASH".to_string()
        })
        .build()
        .into_daemon()?;

    let data = daemon.verify(&ctx).await?;
    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn sql_parameter_substitution_fails_if_parameter_missing() -> Result<(), DataError> {
    let daemon = TestDaemon::builder()
        .expression("SELECT t.digest FROM transactions t WHERE t.digest = '{{ digest }}'")
        .build()
        .into_daemon();

    assert!(matches!(daemon, Err(DataError::RenderSql(_))));

    Ok(())
}

#[test(tokio::test)]
async fn select_report_incident() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");

    let daemon = TestDaemon::builder()
        .expression(
            r#"
            SELECT report('some-tx-hash', 'info', ct.function) FROM call_traces ct
        "#,
        )
        .build()
        .set_core_version("0.1.0".parse().unwrap())
        .into_daemon()?;

    let data = daemon.verify(&ctx).await?;

    // There are two call traces in the test data
    expect![[r#"
        [
            Incident {
                severity: Info,
                message: "func1",
                tx_hash: "some-tx-hash",
                address: "",
                data: [],
            },
            Incident {
                severity: Info,
                message: "func2",
                tx_hash: "some-tx-hash",
                address: "",
                data: [],
            },
        ]
    "#]]
    .assert_debug_eq(&data.incidents);

    Ok(())
}

#[test(tokio::test)]
async fn select_report_incident_full() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");

    let daemon = TestDaemon::builder()
        .expression(
            r#"
        SELECT
            report_full(
                'some-tx-hash',
                'info',
                concat('hey-', t.seq),
                'some-address',
                t.input
            )
        FROM transactions t
    "#,
        )
        .build()
        .set_core_version("0.1.0".parse().unwrap())
        .into_daemon()?;

    let data = daemon.verify(&ctx).await?;

    // There are two call traces in the test data
    expect![[r#"
        [
            Incident {
                severity: Info,
                message: "hey-42",
                tx_hash: "some-tx-hash",
                address: "some-address",
                data: [
                    64,
                    193,
                    15,
                    25,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    85,
                    254,
                    0,
                    42,
                    239,
                    240,
                    47,
                    119,
                    54,
                    77,
                    227,
                    57,
                    161,
                    41,
                    41,
                    35,
                    161,
                    88,
                    68,
                    184,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    71,
                    134,
                    140,
                    0,
                ],
            },
        ]
    "#]]
    .assert_debug_eq(&data.incidents);

    Ok(())
}
