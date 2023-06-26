use maplit::hashmap;
use test_log::test;

use mamoru_core::{DaemonParameters, DataError};
use mamoru_core_test_utils::test_blockchain_data::data_ctx;

use crate::daemon::{test_sql_daemon, try_test_sql_daemon};

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
        let rule = try_test_sql_daemon(statement, DaemonParameters::default());

        assert!(matches!(rule, Err(DataError::UnsupportedStatement)));
    }

    Ok(())
}

#[test(tokio::test)]
async fn sql_parameter_substitution_works() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let daemon = try_test_sql_daemon(
        "SELECT t.digest FROM transactions t WHERE t.digest = '{{ digest }}'",
        hashmap! {
            "digest".to_string() => "DUMMY_HASH".to_string()
        },
    )?;

    let data = daemon.verify(&ctx).await?;
    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn sql_parameter_substitution_fails_if_parameter_missing() -> Result<(), DataError> {
    let daemon = try_test_sql_daemon(
        "SELECT t.digest FROM transactions t WHERE t.digest = '{{ digest }}'",
        DaemonParameters::default(),
    );

    assert!(matches!(daemon, Err(DataError::RenderSql(_))));

    Ok(())
}
