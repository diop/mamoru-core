use test_log::test;

use mamoru_core::DataError;
use mamoru_core_test_utils::test_blockchain_data::data_ctx;

use crate::daemon::test_sql_daemon;

// commit(bytes32)
// 0	commitment	bytes32	0x382d58acbd06cade4c706e0222d56eccb1b095f7d39a776631e68d40aa4781dc
const TEST_TX_INPUT_1: &str =
    "0xf14fcbc8382d58acbd06cade4c706e0222d56eccb1b095f7d39a776631e68d40aa4781dc";

// swap(string,address,uint256,bytes)
// 0	aggregatorId    string	airswapLight3FeeDynamic
// 1	tokenFrom       address	0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
// 2	amount          uint256	60000000000
// 3	data	        bytes	0x00000000000000000000000000000000000000000000000000000000008d4c6b0000000000000000000000000000000000000000000000000000000064a5465a000000000000000000000000b3c839dbde6b96d37c56ee4f9dad3390d49310aa000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec70000000000000000000000000000000000000000000000000000000dd90bbf5c000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000000000000000000000000000000000dddb4f9c0000000000000000000000000000000000000000000000000000000000000001b71257811673ddcf3a46fede2590ad13d4fce36fd09c6b5ba058e3e4502b8da8b358dea0c1d5aca55b43d6ecfcb79c6653efbcf9719eac86eef0c1d1ab0a55062000000000000000000000000000000000000000000000000000000001a925e400000000000000000000000002acf35c9a3f4c5c3f4c78ef5fb64c3ee82f07c450000000000000000000000000000000000000000000000000000000000000000
const TEST_TX_INPUT_2: &str = "0x5f5755290000000000000000000000000000000000000000000000000000000000000080000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000000000000000000000000000000000df847580000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000017616972737761704c696768743346656544796e616d696300000000000000000000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000008d4c6b0000000000000000000000000000000000000000000000000000000064a5465a000000000000000000000000b3c839dbde6b96d37c56ee4f9dad3390d49310aa000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec70000000000000000000000000000000000000000000000000000000dd90bbf5c000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000000000000000000000000000000000dddb4f9c0000000000000000000000000000000000000000000000000000000000000001b71257811673ddcf3a46fede2590ad13d4fce36fd09c6b5ba058e3e4502b8da8b358dea0c1d5aca55b43d6ecfcb79c6653efbcf9719eac86eef0c1d1ab0a55062000000000000000000000000000000000000000000000000000000001a925e400000000000000000000000002acf35c9a3f4c5c3f4c78ef5fb64c3ee82f07c45000000000000000000000000000000000000000000000000000000000000000001b3";
const TEST_TX_INPUT_2_DATA: &str = "0x8d4c6b0000000000000000000000000000000000000000000000000000000064a5465a000000000000000000000000b3c839dbde6b96d37c56ee4f9dad3390d49310aa000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec70000000000000000000000000000000000000000000000000000000dd90bbf5c000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000000000000000000000000000000000dddb4f9c0000000000000000000000000000000000000000000000000000000000000001b71257811673ddcf3a46fede2590ad13d4fce36fd09c6b5ba058e3e4502b8da8b358dea0c1d5aca55b43d6ecfcb79c6653efbcf9719eac86eef0c1d1ab0a55062000000000000000000000000000000000000000000000000000000001a925e400000000000000000000000002acf35c9a3f4c5c3f4c78ef5fb64c3ee82f07c450000000000000000000000000000000000000000000000000000000000000000";

// addIncident(string,tuple)
// 1	daemonId	        string	5e4e6c89-a6c9-4857-8f23-b6f0e06b5112
// 1	incident.IncidentId	string	0xd426eb26581e233b694c8852801c7619120a59d0761b788477a8e982ce144d59-5e4e6c89-a6c9-4857-8f23-b6f0e06b5112
// 1	incident.DaemonId	string	5e4e6c89-a6c9-4857-8f23-b6f0e06b5112
// 1	incident.Address	address	0x0000000000000000000000000000000000000000
// 1	incident.Data	    bytes	0x
// 1	incident.Severity	int32	2
// 1	incident.CreatedAt	uint64	1685616419656
const TEST_TX_INPUT_3: &str = "0x5df1695e000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000002435653465366338392d613663392d343835372d386632332d6236663065303662353131320000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000160000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001c00000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000001887691634800000000000000000000000000000000000000000000000000000000000000673078643432366562323635383165323333623639346338383532383031633736313931323061353964303736316237383834373761386539383263653134346435392d35653465366338392d613663392d343835372d386632332d62366630653036623531313200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002435653465366338392d613663392d343835372d386632332d623666306530366235313132000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

// sellToUniswap(address[],uint256,uint256,bool)
// 0	tokens	        address[]	0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee 0x4c11249814f11b9346808179cf06e71ac328c1b5
// 1	sellAmount	    uint256	    1668386993541889
// 2	minBuyAmount	uint256	    1143705677940019438
// 3	isSushi	        bool	    false
const TEST_TX_INPUT_4: &str = "0xd9627aa400000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000005ed639e1913010000000000000000000000000000000000000000000000000fdf4241253650ee00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000004c11249814f11b9346808179cf06e71ac328c1b5869584cd000000000000000000000000382ffce2287252f930e1c8dc9328dac5bf282ba10000000000000000000000000000000000000000000000e453e5c29d64a56889";

#[test(tokio::test)]
async fn evm_parse_tx_input() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        FROM transactions t
        WHERE
            evm_as_address(
                evm_take_token(
                    evm_parse_tx_input(t.input, 'mint(address,uint256)'),
                    0
                )
            ) = '0x55fe002aeff02f77364de339a1292923a15844b8'
        AND
            evm_as_uint256(
                evm_take_token(
                    evm_parse_tx_input(t.input, 'mint(address,uint256)'),
                    1
                )
            ) = u256_from_str('1200000000')
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn evm_parse_tx_input_wrong_signature() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        FROM transactions t
        WHERE
            evm_as_address(
                evm_take_token(
                    evm_parse_tx_input(t.input, 'mint(address,address)'),
                    0
                )
            ) = '0x55fe002aeff02f77364de339a1292923a15844b8'
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(!data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn evm_parse_tx_input_wrong_index() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(
        r#"
        SELECT 1
        FROM transactions t
        WHERE NOT
            evm_as_address(
                evm_take_token(
                    evm_parse_tx_input(t.input, 'mint(address,uint256)'),
                    2
                )
            ) = '0x55fe002aeff02f77364de339a1292923a15844b8'
    "#,
    );

    let data = rule.verify(&ctx).await?;

    assert!(!data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn evm_as_boolean() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(format!(
        r#"
        WITH test_tx AS (
            SELECT hex_to_bytes('{TEST_TX_INPUT_4}') AS input
        )

        SELECT 1
        FROM test_tx t
        WHERE
            evm_as_boolean(
                evm_take_token(
                    evm_parse_tx_input(t.input, 'sellToUniswap(address[],uint256,uint256,bool)'),
                    3
                )
            ) = false
    "#
    ));

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn evm_as_fixed_bytes() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(format!(
        r#"
        WITH test_tx AS (
            SELECT hex_to_bytes('{TEST_TX_INPUT_1}') AS input
        )

        SELECT 1
        FROM test_tx
        WHERE
            bytes_to_hex(
                evm_as_fixed_bytes(
                    evm_take_token(
                        evm_parse_tx_input(test_tx.input, 'commit(bytes32)'),
                        0
                    )
                )
            ) = '0x382d58acbd06cade4c706e0222d56eccb1b095f7d39a776631e68d40aa4781dc'
    "#
    ));

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn evm_as_bytes() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(format!(
        r#"
        WITH test_tx AS (
            SELECT hex_to_bytes('{TEST_TX_INPUT_2}') AS input
        )

        SELECT 1
        FROM test_tx
        WHERE
            bytes_to_hex(
                evm_as_bytes(
                    evm_take_token(
                        evm_parse_tx_input(test_tx.input, 'swap(string,address,uint256,bytes)'),
                        3
                    )
                )
            ) = '{TEST_TX_INPUT_2_DATA}'
        "#
    ));

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn evm_as_string() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(format!(
        r#"
        WITH test_tx AS (
            SELECT hex_to_bytes('{TEST_TX_INPUT_2}') AS input
        )

        SELECT 1
        FROM test_tx
        WHERE
            evm_as_string(
                evm_take_token(
                    evm_parse_tx_input(test_tx.input, 'swap(string,address,uint256,bytes)'),
                    0
                )
            ) = 'airswapLight3FeeDynamic'
    "#
    ));

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn evm_as_uint256() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(format!(
        r#"
        WITH test_tx AS (
            SELECT hex_to_bytes('{TEST_TX_INPUT_2}') AS input
        )

        SELECT 1
        FROM test_tx
        WHERE
            evm_as_uint256(
                evm_take_token(
                    evm_parse_tx_input(test_tx.input, 'swap(string,address,uint256,bytes)'),
                    2
                )
            ) = u256_from_str('60000000000')
        "#
    ));

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn evm_as_tuple() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(format!(
        r#"
        WITH test_tx AS (
            SELECT hex_to_bytes('{TEST_TX_INPUT_3}') AS input
        ),

        daemon AS (
            SELECT
                evm_take_token(
                    evm_as_tuple(
                        evm_take_token(
                            evm_parse_tx_input(test_tx.input, 'addIncident(string,(string,string,address,bytes,int32,uint64))'),
                            1
                        )
                    ),
                    1
                ) as id
             FROM test_tx
        )

        SELECT 1 FROM daemon WHERE evm_as_string(daemon.id) = '5e4e6c89-a6c9-4857-8f23-b6f0e06b5112'
        "#
    ));

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}

#[test(tokio::test)]
async fn evm_as_array() -> Result<(), DataError> {
    let ctx = data_ctx("DUMMY_HASH");
    let rule = test_sql_daemon(format!(
        r#"
        WITH test_tx AS (
            SELECT hex_to_bytes('{TEST_TX_INPUT_4}') AS input
        ),

        tokens AS (
            SELECT
                evm_as_array(
                    evm_take_token(
                        evm_parse_tx_input(t.input, 'sellToUniswap(address[],uint256,uint256,bool)'),
                        0
                    )
                ) as ids
            FROM test_tx t
        )

        SELECT 1
        FROM tokens t
        WHERE
            evm_as_address(
                evm_take_token(
                    t.ids,
                    0
                )
            ) = '0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee'
        AND
            evm_as_address(
                evm_take_token(
                    t.ids,
                    1
                )
            ) = '0x4c11249814f11b9346808179cf06e71ac328c1b5'
    "#
    ));

    let data = rule.verify(&ctx).await?;

    assert!(data.matched);

    Ok(())
}
