use test_log::test;

use mamoru_core::DataError;
use mamoru_core_test_utils::test_blockchain_data::{data_ctx, TEST_ETH_TOPIC};

use crate::daemon::test_sql_daemon;

#[test(tokio::test)]
async fn uint64_type_conversion() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon("SELECT 1 FROM call_trace_args cta WHERE as_uint64(cta.arg) = 42;");

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn bool_type_conversion() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule =
        test_sql_daemon("SELECT 1 FROM call_trace_args cta WHERE as_boolean(cta.arg) = false;");

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn get_struct_field() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        "SELECT 1 FROM call_trace_args cta WHERE as_uint64(struct_field(cta.arg, 'foo')) = 42;",
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn wrong_struct_field_does_not_fail() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        "SELECT 1 FROM call_trace_args cta WHERE as_uint64(struct_field(cta.arg, 'does-not-exist')) = 42;",
    );

    let data = rule.verify(&ctx).await?;

    assert!(!data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn bytes_to_hex() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(format!(
        "SELECT 1 FROM transactions t WHERE bytes_to_hex(t.eth_topic) = '0x{}'",
        TEST_ETH_TOPIC
    ));

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn u256_from_str() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon("SELECT 1 WHERE bytes_to_hex(u256_from_str('10')) = '0xa';");

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn u256_eq() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            u256_eq(u256_from_str('10'), u256_from_str('10'))
        AND NOT
            u256_eq(u256_from_str('11'), u256_from_str('10'));

    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn u256_gt() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            u256_gt(u256_from_str('11'), u256_from_str('10'))
        AND NOT
            u256_gt(u256_from_str('10'), u256_from_str('10'))
        AND NOT
            u256_gt(u256_from_str('9'), u256_from_str('10'));
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn u256_ge() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            u256_ge(u256_from_str('11'), u256_from_str('10'))
        AND
            u256_ge(u256_from_str('10'), u256_from_str('10'))
        AND NOT
            u256_ge(u256_from_str('9'), u256_from_str('10'));
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn u256_lt() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            u256_lt(u256_from_str('9'), u256_from_str('10'))
        AND NOT
            u256_lt(u256_from_str('10'), u256_from_str('10'))
        AND NOT
            u256_lt(u256_from_str('11'), u256_from_str('10'));
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn u256_le() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            u256_le(u256_from_str('9'), u256_from_str('10'))
        AND
            u256_le(u256_from_str('10'), u256_from_str('10'))
        AND NOT
            u256_le(u256_from_str('11'), u256_from_str('10'));
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn u256_add() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            bytes_to_hex(
                u256_add(
                    u256_from_str('9'),
                    u256_from_str('1')
                )
            ) = '0xa'
        AND
            u256_eq(
                u256_add(
                    u256_from_str('0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff'),
                    u256_from_str('1')
                ),
                u256_from_str('0')
            )
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn u256_sub() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            bytes_to_hex(u256_sub(u256_from_str('0xa'), u256_from_str('1'))) = '0x9'
        AND
            u256_eq(
                u256_sub(
                    u256_from_str('0'),
                    u256_from_str('1')
                ),
                u256_from_str('0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff')
            );
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn u256_mul() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            bytes_to_hex(
                u256_mul(
                    u256_from_str('10'),
                    u256_from_str('10')
                )
            ) = '0x64'
        "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn u256_div() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            bytes_to_hex(
                u256_div(
                    u256_from_str('10'),
                    u256_from_str('10')
                )
            ) = '0x1'
        AND
            bytes_to_hex(
                u256_div(
                    u256_from_str('10'),
                    u256_from_str('3')
                )
            ) = '0x3'
        AND
            bytes_to_hex(
                u256_div(
                    u256_from_str('10'),
                    u256_from_str('2')
                )
            ) = '0x5'
        "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn u256_pow() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            bytes_to_hex(
                u256_pow(
                    u256_from_str('10'),
                    10
                )
            ) = '0x2540be400'
        AND
            bytes_to_hex(
                u256_pow(
                    u256_from_str('10'),
                    0
                )
            ) = '0x1'
        AND
            bytes_to_hex(
                u256_pow(
                    u256_from_str('10'),
                    1
                )
            ) = '0xa'
        AND
            bytes_to_hex(
                u256_pow(
                    u256_from_str('10'),
                    2
                )
            ) = '0x64'
        "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn i256_from_str() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon("SELECT 1 WHERE bytes_to_hex(i256_from_str('-10')) = '0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6'");

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn i256_eq() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            i256_eq(i256_from_str('10'), i256_from_str('10'))
        AND NOT
            i256_eq(i256_from_str('11'), i256_from_str('10'));

    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn i256_gt() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            i256_gt(i256_from_str('11'), i256_from_str('10'))
        AND NOT
            i256_gt(i256_from_str('10'), i256_from_str('10'))
        AND NOT
            i256_gt(i256_from_str('9'), i256_from_str('10'));
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn i256_ge() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            i256_ge(i256_from_str('11'), i256_from_str('10'))
        AND
            i256_ge(i256_from_str('10'), i256_from_str('10'))
        AND NOT
            i256_ge(i256_from_str('9'), i256_from_str('10'));
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn i256_lt() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            i256_lt(i256_from_str('9'), i256_from_str('10'))
        AND NOT
            i256_lt(i256_from_str('10'), i256_from_str('10'))
        AND NOT
            i256_lt(i256_from_str('11'), i256_from_str('10'));
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn i256_le() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            i256_le(i256_from_str('9'), i256_from_str('10'))
        AND
            i256_le(i256_from_str('10'), i256_from_str('10'))
        AND NOT
            i256_le(i256_from_str('11'), i256_from_str('10'));
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn i256_add() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            bytes_to_hex(i256_add(i256_from_str('9'), i256_from_str('1'))) = '0xa'
        AND
            i256_eq(
                i256_add(
                    i256_from_str('-1'),
                    i256_from_str('1')
                ),
                i256_from_str('0')
            );
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn i256_sub() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            bytes_to_hex(
                i256_sub(
                    i256_from_str('0xa'),
                    i256_from_str('1')
                )
            ) = '0x9'
        AND
            i256_eq(
                i256_sub(
                    i256_from_str('0'),
                    i256_from_str('1')
                ),
                i256_from_str('-1')
            );
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn i256_mul() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            bytes_to_hex(
                i256_mul(
                    i256_from_str('10'),
                    i256_from_str('10')
                )
            ) = '0x64'
        AND
            i256_eq(
                i256_mul(
                    i256_from_str('-1'),
                    i256_from_str('1')
                ),
                i256_from_str('-1')
            );
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn i256_div() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");

    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            bytes_to_hex(
                i256_div(
                    i256_from_str('10'),
                    i256_from_str('10')
                )
            ) = '0x1'
        AND
            bytes_to_hex(
                i256_div(
                    i256_from_str('10'),
                    i256_from_str('3')
                )
            ) = '0x3'
        AND
            bytes_to_hex(
                i256_div(
                    i256_from_str('10'),
                    i256_from_str('2')
                )
            ) = '0x5'
        AND
            i256_eq(
                i256_div(
                    i256_from_str('-1'),
                    i256_from_str('1')
                ),
                i256_from_str('-1')
            );
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn i256_pow() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");

    let rule = test_sql_daemon(
        r#"
        SELECT 1
        WHERE
            bytes_to_hex(
                i256_pow(
                    i256_from_str('10'),
                    10
                )
            ) = '0x2540be400'
        AND
            bytes_to_hex(
                i256_pow(
                    i256_from_str('10'),
                    0
                )
            ) = '0x1'
        AND
            bytes_to_hex(
                i256_pow(
                    i256_from_str('10'),
                    1
                )
            ) = '0xa'
        AND
            i256_eq(
                i256_pow(
                    i256_from_str('-1'),
                    1
                ),
                i256_from_str('-1')
            );
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}
