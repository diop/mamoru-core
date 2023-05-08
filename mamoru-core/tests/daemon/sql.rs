use test_log::test;

use mamoru_core::DataError;
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
        let rule = try_test_sql_daemon(statement);

        assert!(matches!(rule, Err(DataError::UnsupportedStatement)));
    }

    Ok(())
}
