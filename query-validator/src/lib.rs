pub use error::*;
use mamoru_aptos_types::AptosCtx;
use mamoru_core::{
    BlockchainCtx, BlockchainData, BlockchainDataBuilder, Daemon, DaemonParameters, DataError,
    IncidentData, IncidentSeverity,
};
use mamoru_evm_types::EvmCtx;
use mamoru_sui_types::SuiCtx;

mod error;

/// Represents possible blockchains as each one has different schema
#[derive(Debug)]
pub enum ChainType {
    Sui,
    Evm,
    Aptos,
}

/// Validates an SQL Daemon query against an empty database.
pub async fn validate_sql(chain: ChainType, query: &str) -> Result<(), ValidateError> {
    let result = match chain {
        ChainType::Sui => {
            let ctx = empty_ctx::<SuiCtx>();
            let daemon = sql_validation_daemon(query)?;

            daemon.verify(&ctx).await?
        }
        ChainType::Evm => {
            let ctx = empty_ctx::<EvmCtx>();
            let daemon = sql_validation_daemon(query)?;

            daemon.verify(&ctx).await?
        }
        ChainType::Aptos => {
            let ctx = empty_ctx::<AptosCtx>();
            let daemon = sql_validation_daemon(query)?;

            daemon.verify(&ctx).await?
        }
    };

    if result.matched {
        return Err(ValidateError::MatchesEmptyDatabase);
    }

    Ok(())
}

/// Validates a AssemblyScript Daemon against an empty database.
pub async fn validate_assembly_script(chain: ChainType, bytes: &[u8]) -> Result<(), ValidateError> {
    let result = match chain {
        ChainType::Sui => {
            let ctx = empty_ctx::<SuiCtx>();
            let daemon = assembly_script_validation_daemon(bytes)?;

            daemon.verify(&ctx).await?
        }
        ChainType::Evm => {
            let ctx = empty_ctx::<EvmCtx>();
            let daemon = assembly_script_validation_daemon(bytes)?;

            daemon.verify(&ctx).await?
        }
        ChainType::Aptos => {
            let ctx = empty_ctx::<AptosCtx>();
            let daemon = assembly_script_validation_daemon(bytes)?;

            daemon.verify(&ctx).await?
        }
    };
    if result.matched {
        return Err(ValidateError::MatchesEmptyDatabase);
    }

    Ok(())
}

fn sql_validation_daemon(query: &str) -> Result<Daemon, DataError> {
    Daemon::new_sql(
        "QUERY_VALIDATE".to_string(),
        query,
        IncidentData {
            message: "QUERY_VALIDATE".to_string(),
            severity: IncidentSeverity::Info,
        },
    )
}

fn assembly_script_validation_daemon(bytes: &[u8]) -> Result<Daemon, DataError> {
    Daemon::new_assembly_script("WASM_VALIDATE".to_string(), bytes, DaemonParameters::new())
}

const EMPTY_CTX: &str = "EMPTY_CTX";

fn empty_ctx<T: BlockchainCtx>() -> BlockchainData<T> {
    BlockchainDataBuilder::<T>::new()
        .build(EMPTY_CTX, EMPTY_CTX)
        .unwrap_or_else(|_| {
            panic!(
                "BUG: `ChainCtxBuilder::<{}>::new().build` fails.",
                std::any::type_name::<T>(),
            )
        })
}

#[cfg(test)]
mod tests {
    use mamoru_evm_types::EvmCtx;

    use super::*;

    #[test]
    fn sui_empty_ctx_does_not_fail() {
        empty_ctx::<SuiCtx>();
    }

    #[test]
    fn evm_empty_ctx_does_not_fail() {
        empty_ctx::<EvmCtx>();
    }

    #[test]
    fn aptos_empty_ctx_does_not_fail() {
        empty_ctx::<AptosCtx>();
    }

    #[tokio::test]
    async fn valid_expression_ok() {
        let result = validate_sql(ChainType::Sui, "SELECT * FROM transactions").await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn always_true_expression_fails() {
        let result = validate_sql(ChainType::Sui, "SELECT 1").await;

        assert!(matches!(result, Err(ValidateError::MatchesEmptyDatabase)))
    }

    #[tokio::test]
    async fn wrong_table_name_fails() {
        let result = validate_sql(ChainType::Sui, "SELECT * FROM THIS_TABLE_DOES_NOT_EXIST").await;

        assert!(matches!(
            result,
            Err(ValidateError::DataError(DataError::PlanQuery(_)))
        ))
    }

    #[tokio::test]
    async fn minimum_valid_assembly_script_ok() {
        let result = validate_assembly_script(ChainType::Sui, ASC_EMPTY_MAIN).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn minimum_valid_assembly_script_without_runtime_fails() {
        let result = validate_assembly_script(ChainType::Sui, ASC_EMPTY_MAIN_NO_RUNTIME).await;

        assert!(matches!(
            result,
            Err(ValidateError::DataError(DataError::WasmExport { .. }))
        ));
    }

    // export function main(): void {}
    const ASC_EMPTY_MAIN_NO_RUNTIME: &[u8] = &[
        0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 3, 1, 0, 0, 7, 17, 2, 4,
        109, 97, 105, 110, 0, 0, 6, 109, 101, 109, 111, 114, 121, 2, 0, 10, 5, 1, 3, 0, 1, 11, 0,
        36, 16, 115, 111, 117, 114, 99, 101, 77, 97, 112, 112, 105, 110, 103, 85, 82, 76, 18, 46,
        47, 114, 101, 108, 101, 97, 115, 101, 46, 119, 97, 115, 109, 46, 109, 97, 112,
    ];

    // export function main(): void {} with `--exportRuntime`
    const ASC_EMPTY_MAIN: &[u8] = &[
        0, 97, 115, 109, 1, 0, 0, 0, 1, 41, 8, 96, 0, 0, 96, 1, 127, 0, 96, 2, 127, 127, 0, 96, 2,
        127, 127, 1, 127, 96, 4, 127, 127, 127, 127, 0, 96, 3, 127, 127, 127, 0, 96, 0, 1, 127, 96,
        1, 127, 1, 127, 2, 13, 1, 3, 101, 110, 118, 5, 97, 98, 111, 114, 116, 0, 4, 3, 18, 17, 0,
        0, 1, 1, 2, 2, 5, 0, 6, 3, 3, 7, 1, 0, 1, 0, 1, 5, 3, 1, 0, 1, 6, 57, 11, 127, 1, 65, 0,
        11, 127, 1, 65, 0, 11, 127, 1, 65, 0, 11, 127, 1, 65, 0, 11, 127, 1, 65, 0, 11, 127, 1, 65,
        0, 11, 127, 1, 65, 0, 11, 127, 1, 65, 0, 11, 127, 1, 65, 0, 11, 127, 1, 65, 0, 11, 127, 0,
        65, 160, 12, 11, 7, 69, 7, 4, 109, 97, 105, 110, 0, 1, 5, 95, 95, 110, 101, 119, 0, 11, 5,
        95, 95, 112, 105, 110, 0, 12, 7, 95, 95, 117, 110, 112, 105, 110, 0, 13, 9, 95, 95, 99,
        111, 108, 108, 101, 99, 116, 0, 14, 11, 95, 95, 114, 116, 116, 105, 95, 98, 97, 115, 101,
        3, 10, 6, 109, 101, 109, 111, 114, 121, 2, 0, 8, 1, 16, 12, 1, 15, 10, 223, 24, 17, 3, 0,
        1, 11, 93, 1, 2, 127, 65, 224, 9, 16, 17, 65, 160, 8, 16, 17, 65, 176, 11, 16, 17, 65, 240,
        11, 16, 17, 35, 4, 34, 1, 40, 2, 4, 65, 124, 113, 33, 0, 3, 64, 32, 0, 32, 1, 71, 4, 64,
        32, 0, 40, 2, 4, 65, 3, 113, 65, 3, 71, 4, 64, 65, 0, 65, 224, 8, 65, 160, 1, 65, 16, 16,
        0, 0, 11, 32, 0, 65, 20, 106, 16, 15, 32, 0, 40, 2, 4, 65, 124, 113, 33, 0, 12, 1, 11, 11,
        11, 97, 1, 1, 127, 32, 0, 40, 2, 4, 65, 124, 113, 34, 1, 69, 4, 64, 32, 0, 40, 2, 8, 69,
        32, 0, 65, 180, 140, 2, 73, 113, 69, 4, 64, 65, 0, 65, 224, 8, 65, 128, 1, 65, 18, 16, 0,
        0, 11, 15, 11, 32, 0, 40, 2, 8, 34, 0, 69, 4, 64, 65, 0, 65, 224, 8, 65, 132, 1, 65, 16,
        16, 0, 0, 11, 32, 1, 32, 0, 54, 2, 8, 32, 0, 32, 1, 32, 0, 40, 2, 4, 65, 3, 113, 114, 54,
        2, 4, 11, 159, 1, 1, 3, 127, 32, 0, 35, 5, 70, 4, 64, 32, 0, 40, 2, 8, 34, 1, 69, 4, 64,
        65, 0, 65, 224, 8, 65, 148, 1, 65, 30, 16, 0, 0, 11, 32, 1, 36, 5, 11, 32, 0, 16, 3, 35, 6,
        33, 1, 32, 0, 40, 2, 12, 34, 2, 65, 2, 77, 4, 127, 65, 1, 5, 32, 2, 65, 160, 12, 40, 2, 0,
        75, 4, 64, 65, 224, 9, 65, 160, 10, 65, 21, 65, 28, 16, 0, 0, 11, 32, 2, 65, 2, 116, 65,
        164, 12, 106, 40, 2, 0, 65, 32, 113, 11, 33, 3, 32, 1, 40, 2, 8, 33, 2, 32, 0, 35, 7, 69,
        65, 2, 32, 3, 27, 32, 1, 114, 54, 2, 4, 32, 0, 32, 2, 54, 2, 8, 32, 2, 32, 0, 32, 2, 40, 2,
        4, 65, 3, 113, 114, 54, 2, 4, 32, 1, 32, 0, 54, 2, 8, 11, 148, 2, 1, 4, 127, 32, 1, 40, 2,
        0, 34, 2, 65, 1, 113, 69, 4, 64, 65, 0, 65, 240, 10, 65, 140, 2, 65, 14, 16, 0, 0, 11, 32,
        2, 65, 124, 113, 34, 2, 65, 12, 73, 4, 64, 65, 0, 65, 240, 10, 65, 142, 2, 65, 14, 16, 0,
        0, 11, 32, 2, 65, 128, 2, 73, 4, 127, 32, 2, 65, 4, 118, 5, 65, 31, 65, 252, 255, 255, 255,
        3, 32, 2, 32, 2, 65, 252, 255, 255, 255, 3, 79, 27, 34, 2, 103, 107, 34, 4, 65, 7, 107, 33,
        3, 32, 2, 32, 4, 65, 4, 107, 118, 65, 16, 115, 11, 34, 2, 65, 16, 73, 32, 3, 65, 23, 73,
        113, 69, 4, 64, 65, 0, 65, 240, 10, 65, 156, 2, 65, 14, 16, 0, 0, 11, 32, 1, 40, 2, 8, 33,
        5, 32, 1, 40, 2, 4, 34, 4, 4, 64, 32, 4, 32, 5, 54, 2, 8, 11, 32, 5, 4, 64, 32, 5, 32, 4,
        54, 2, 4, 11, 32, 1, 32, 0, 32, 3, 65, 4, 116, 32, 2, 106, 65, 2, 116, 106, 40, 2, 96, 70,
        4, 64, 32, 0, 32, 3, 65, 4, 116, 32, 2, 106, 65, 2, 116, 106, 32, 5, 54, 2, 96, 32, 5, 69,
        4, 64, 32, 0, 32, 3, 65, 2, 116, 106, 34, 1, 40, 2, 4, 65, 126, 32, 2, 119, 113, 33, 2, 32,
        1, 32, 2, 54, 2, 4, 32, 2, 69, 4, 64, 32, 0, 32, 0, 40, 2, 0, 65, 126, 32, 3, 119, 113, 54,
        2, 0, 11, 11, 11, 11, 195, 3, 1, 5, 127, 32, 1, 69, 4, 64, 65, 0, 65, 240, 10, 65, 201, 1,
        65, 14, 16, 0, 0, 11, 32, 1, 40, 2, 0, 34, 3, 65, 1, 113, 69, 4, 64, 65, 0, 65, 240, 10,
        65, 203, 1, 65, 14, 16, 0, 0, 11, 32, 1, 65, 4, 106, 32, 1, 40, 2, 0, 65, 124, 113, 106,
        34, 4, 40, 2, 0, 34, 2, 65, 1, 113, 4, 64, 32, 0, 32, 4, 16, 5, 32, 1, 32, 3, 65, 4, 106,
        32, 2, 65, 124, 113, 106, 34, 3, 54, 2, 0, 32, 1, 65, 4, 106, 32, 1, 40, 2, 0, 65, 124,
        113, 106, 34, 4, 40, 2, 0, 33, 2, 11, 32, 3, 65, 2, 113, 4, 64, 32, 1, 65, 4, 107, 40, 2,
        0, 34, 1, 40, 2, 0, 34, 6, 65, 1, 113, 69, 4, 64, 65, 0, 65, 240, 10, 65, 221, 1, 65, 16,
        16, 0, 0, 11, 32, 0, 32, 1, 16, 5, 32, 1, 32, 6, 65, 4, 106, 32, 3, 65, 124, 113, 106, 34,
        3, 54, 2, 0, 11, 32, 4, 32, 2, 65, 2, 114, 54, 2, 0, 32, 3, 65, 124, 113, 34, 2, 65, 12,
        73, 4, 64, 65, 0, 65, 240, 10, 65, 233, 1, 65, 14, 16, 0, 0, 11, 32, 4, 32, 1, 65, 4, 106,
        32, 2, 106, 71, 4, 64, 65, 0, 65, 240, 10, 65, 234, 1, 65, 14, 16, 0, 0, 11, 32, 4, 65, 4,
        107, 32, 1, 54, 2, 0, 32, 2, 65, 128, 2, 73, 4, 127, 32, 2, 65, 4, 118, 5, 65, 31, 65, 252,
        255, 255, 255, 3, 32, 2, 32, 2, 65, 252, 255, 255, 255, 3, 79, 27, 34, 2, 103, 107, 34, 3,
        65, 7, 107, 33, 5, 32, 2, 32, 3, 65, 4, 107, 118, 65, 16, 115, 11, 34, 2, 65, 16, 73, 32,
        5, 65, 23, 73, 113, 69, 4, 64, 65, 0, 65, 240, 10, 65, 251, 1, 65, 14, 16, 0, 0, 11, 32, 0,
        32, 5, 65, 4, 116, 32, 2, 106, 65, 2, 116, 106, 40, 2, 96, 33, 3, 32, 1, 65, 0, 54, 2, 4,
        32, 1, 32, 3, 54, 2, 8, 32, 3, 4, 64, 32, 3, 32, 1, 54, 2, 4, 11, 32, 0, 32, 5, 65, 4, 116,
        32, 2, 106, 65, 2, 116, 106, 32, 1, 54, 2, 96, 32, 0, 32, 0, 40, 2, 0, 65, 1, 32, 5, 116,
        114, 54, 2, 0, 32, 0, 32, 5, 65, 2, 116, 106, 34, 0, 32, 0, 40, 2, 4, 65, 1, 32, 2, 116,
        114, 54, 2, 4, 11, 205, 1, 1, 2, 127, 32, 1, 32, 2, 75, 4, 64, 65, 0, 65, 240, 10, 65, 249,
        2, 65, 14, 16, 0, 0, 11, 32, 1, 65, 19, 106, 65, 112, 113, 65, 4, 107, 33, 1, 32, 0, 40, 2,
        160, 12, 34, 4, 4, 64, 32, 4, 65, 4, 106, 32, 1, 75, 4, 64, 65, 0, 65, 240, 10, 65, 128, 3,
        65, 16, 16, 0, 0, 11, 32, 1, 65, 16, 107, 32, 4, 70, 4, 64, 32, 4, 40, 2, 0, 33, 3, 32, 1,
        65, 16, 107, 33, 1, 11, 5, 32, 0, 65, 164, 12, 106, 32, 1, 75, 4, 64, 65, 0, 65, 240, 10,
        65, 141, 3, 65, 5, 16, 0, 0, 11, 11, 32, 2, 65, 112, 113, 32, 1, 107, 34, 2, 65, 20, 73, 4,
        64, 15, 11, 32, 1, 32, 3, 65, 2, 113, 32, 2, 65, 8, 107, 34, 2, 65, 1, 114, 114, 54, 2, 0,
        32, 1, 65, 0, 54, 2, 4, 32, 1, 65, 0, 54, 2, 8, 32, 1, 65, 4, 106, 32, 2, 106, 34, 2, 65,
        2, 54, 2, 0, 32, 0, 32, 2, 54, 2, 160, 12, 32, 0, 32, 1, 16, 6, 11, 150, 1, 1, 2, 127, 63,
        0, 34, 1, 65, 0, 76, 4, 127, 65, 1, 32, 1, 107, 64, 0, 65, 0, 72, 5, 65, 0, 11, 4, 64, 0,
        11, 65, 192, 140, 2, 65, 0, 54, 2, 0, 65, 224, 152, 2, 65, 0, 54, 2, 0, 3, 64, 32, 0, 65,
        23, 73, 4, 64, 32, 0, 65, 2, 116, 65, 192, 140, 2, 106, 65, 0, 54, 2, 4, 65, 0, 33, 1, 3,
        64, 32, 1, 65, 16, 73, 4, 64, 32, 0, 65, 4, 116, 32, 1, 106, 65, 2, 116, 65, 192, 140, 2,
        106, 65, 0, 54, 2, 96, 32, 1, 65, 1, 106, 33, 1, 12, 1, 11, 11, 32, 0, 65, 1, 106, 33, 0,
        12, 1, 11, 11, 65, 192, 140, 2, 65, 228, 152, 2, 63, 0, 65, 16, 116, 16, 7, 65, 192, 140,
        2, 36, 9, 11, 242, 3, 1, 3, 127, 2, 64, 2, 64, 2, 64, 2, 64, 35, 2, 14, 3, 0, 1, 2, 3, 11,
        65, 1, 36, 2, 65, 0, 36, 3, 16, 2, 35, 6, 36, 5, 35, 3, 15, 11, 35, 7, 69, 33, 1, 35, 5,
        40, 2, 4, 65, 124, 113, 33, 0, 3, 64, 32, 0, 35, 6, 71, 4, 64, 32, 0, 36, 5, 32, 1, 32, 0,
        40, 2, 4, 65, 3, 113, 71, 4, 64, 32, 0, 32, 0, 40, 2, 4, 65, 124, 113, 32, 1, 114, 54, 2,
        4, 65, 0, 36, 3, 32, 0, 65, 20, 106, 16, 15, 35, 3, 15, 11, 32, 0, 40, 2, 4, 65, 124, 113,
        33, 0, 12, 1, 11, 11, 65, 0, 36, 3, 16, 2, 35, 6, 35, 5, 40, 2, 4, 65, 124, 113, 70, 4, 64,
        65, 180, 140, 2, 33, 0, 3, 64, 32, 0, 65, 180, 140, 2, 73, 4, 64, 32, 0, 40, 2, 0, 34, 2,
        4, 64, 32, 2, 16, 17, 11, 32, 0, 65, 4, 106, 33, 0, 12, 1, 11, 11, 35, 5, 40, 2, 4, 65,
        124, 113, 33, 0, 3, 64, 32, 0, 35, 6, 71, 4, 64, 32, 1, 32, 0, 40, 2, 4, 65, 3, 113, 71, 4,
        64, 32, 0, 32, 0, 40, 2, 4, 65, 124, 113, 32, 1, 114, 54, 2, 4, 32, 0, 65, 20, 106, 16, 15,
        11, 32, 0, 40, 2, 4, 65, 124, 113, 33, 0, 12, 1, 11, 11, 35, 8, 33, 0, 35, 6, 36, 8, 32, 0,
        36, 6, 32, 1, 36, 7, 32, 0, 40, 2, 4, 65, 124, 113, 36, 5, 65, 2, 36, 2, 11, 35, 3, 15, 11,
        35, 5, 34, 0, 35, 6, 71, 4, 64, 32, 0, 40, 2, 4, 34, 1, 65, 124, 113, 36, 5, 35, 7, 69, 32,
        1, 65, 3, 113, 71, 4, 64, 65, 0, 65, 224, 8, 65, 229, 1, 65, 20, 16, 0, 0, 11, 32, 0, 65,
        180, 140, 2, 73, 4, 64, 32, 0, 65, 0, 54, 2, 4, 32, 0, 65, 0, 54, 2, 8, 5, 35, 0, 32, 0,
        40, 2, 0, 65, 124, 113, 65, 4, 106, 107, 36, 0, 32, 0, 65, 4, 106, 34, 0, 65, 180, 140, 2,
        79, 4, 64, 35, 9, 69, 4, 64, 16, 8, 11, 35, 9, 33, 1, 32, 0, 65, 4, 107, 33, 2, 32, 0, 65,
        15, 113, 65, 1, 32, 0, 27, 4, 127, 65, 1, 5, 32, 2, 40, 2, 0, 65, 1, 113, 11, 4, 64, 65, 0,
        65, 240, 10, 65, 175, 4, 65, 3, 16, 0, 0, 11, 32, 2, 32, 2, 40, 2, 0, 65, 1, 114, 54, 2, 0,
        32, 1, 32, 2, 16, 6, 11, 11, 65, 10, 15, 11, 35, 6, 34, 0, 32, 0, 54, 2, 4, 32, 0, 32, 0,
        54, 2, 8, 65, 0, 36, 2, 11, 65, 0, 11, 212, 1, 1, 2, 127, 32, 1, 65, 128, 2, 73, 4, 127,
        32, 1, 65, 4, 118, 5, 65, 31, 32, 1, 65, 1, 65, 27, 32, 1, 103, 107, 116, 106, 65, 1, 107,
        32, 1, 32, 1, 65, 254, 255, 255, 255, 1, 73, 27, 34, 1, 103, 107, 34, 3, 65, 7, 107, 33, 2,
        32, 1, 32, 3, 65, 4, 107, 118, 65, 16, 115, 11, 34, 1, 65, 16, 73, 32, 2, 65, 23, 73, 113,
        69, 4, 64, 65, 0, 65, 240, 10, 65, 202, 2, 65, 14, 16, 0, 0, 11, 32, 0, 32, 2, 65, 2, 116,
        106, 40, 2, 4, 65, 127, 32, 1, 116, 113, 34, 1, 4, 127, 32, 0, 32, 1, 104, 32, 2, 65, 4,
        116, 106, 65, 2, 116, 106, 40, 2, 96, 5, 32, 0, 40, 2, 0, 65, 127, 32, 2, 65, 1, 106, 116,
        113, 34, 1, 4, 127, 32, 0, 32, 1, 104, 34, 1, 65, 2, 116, 106, 40, 2, 4, 34, 2, 69, 4, 64,
        65, 0, 65, 240, 10, 65, 215, 2, 65, 18, 16, 0, 0, 11, 32, 0, 32, 2, 104, 32, 1, 65, 4, 116,
        106, 65, 2, 116, 106, 40, 2, 96, 5, 65, 0, 11, 11, 11, 180, 4, 1, 5, 127, 32, 0, 65, 236,
        255, 255, 255, 3, 79, 4, 64, 65, 160, 8, 65, 224, 8, 65, 133, 2, 65, 31, 16, 0, 0, 11, 35,
        0, 35, 1, 79, 4, 64, 2, 64, 65, 128, 16, 33, 2, 3, 64, 32, 2, 16, 9, 107, 33, 2, 35, 2, 69,
        4, 64, 35, 0, 173, 66, 200, 1, 126, 66, 228, 0, 128, 167, 65, 128, 8, 106, 36, 1, 12, 2,
        11, 32, 2, 65, 0, 74, 13, 0, 11, 35, 0, 34, 2, 32, 2, 35, 1, 107, 65, 128, 8, 73, 65, 10,
        116, 106, 36, 1, 11, 11, 35, 9, 69, 4, 64, 16, 8, 11, 35, 9, 33, 4, 32, 0, 65, 16, 106, 34,
        2, 65, 252, 255, 255, 255, 3, 75, 4, 64, 65, 160, 8, 65, 240, 10, 65, 202, 3, 65, 29, 16,
        0, 0, 11, 32, 4, 65, 12, 32, 2, 65, 19, 106, 65, 112, 113, 65, 4, 107, 32, 2, 65, 12, 77,
        27, 34, 5, 16, 10, 34, 2, 69, 4, 64, 63, 0, 34, 2, 65, 4, 32, 4, 40, 2, 160, 12, 32, 2, 65,
        16, 116, 65, 4, 107, 71, 116, 32, 5, 65, 1, 65, 27, 32, 5, 103, 107, 116, 65, 1, 107, 106,
        32, 5, 32, 5, 65, 254, 255, 255, 255, 1, 73, 27, 106, 65, 255, 255, 3, 106, 65, 128, 128,
        124, 113, 65, 16, 118, 34, 3, 32, 2, 32, 3, 74, 27, 64, 0, 65, 0, 72, 4, 64, 32, 3, 64, 0,
        65, 0, 72, 4, 64, 0, 11, 11, 32, 4, 32, 2, 65, 16, 116, 63, 0, 65, 16, 116, 16, 7, 32, 4,
        32, 5, 16, 10, 34, 2, 69, 4, 64, 65, 0, 65, 240, 10, 65, 240, 3, 65, 16, 16, 0, 0, 11, 11,
        32, 5, 32, 2, 40, 2, 0, 65, 124, 113, 75, 4, 64, 65, 0, 65, 240, 10, 65, 242, 3, 65, 14,
        16, 0, 0, 11, 32, 4, 32, 2, 16, 5, 32, 2, 40, 2, 0, 33, 3, 32, 5, 65, 4, 106, 65, 15, 113,
        4, 64, 65, 0, 65, 240, 10, 65, 229, 2, 65, 14, 16, 0, 0, 11, 32, 3, 65, 124, 113, 32, 5,
        107, 34, 6, 65, 16, 79, 4, 64, 32, 2, 32, 5, 32, 3, 65, 2, 113, 114, 54, 2, 0, 32, 2, 65,
        4, 106, 32, 5, 106, 34, 3, 32, 6, 65, 4, 107, 65, 1, 114, 54, 2, 0, 32, 4, 32, 3, 16, 6, 5,
        32, 2, 32, 3, 65, 126, 113, 54, 2, 0, 32, 2, 65, 4, 106, 32, 2, 40, 2, 0, 65, 124, 113,
        106, 34, 3, 32, 3, 40, 2, 0, 65, 125, 113, 54, 2, 0, 11, 32, 2, 32, 1, 54, 2, 12, 32, 2,
        32, 0, 54, 2, 16, 35, 8, 34, 1, 40, 2, 8, 33, 3, 32, 2, 32, 1, 35, 7, 114, 54, 2, 4, 32, 2,
        32, 3, 54, 2, 8, 32, 3, 32, 2, 32, 3, 40, 2, 4, 65, 3, 113, 114, 54, 2, 4, 32, 1, 32, 2,
        54, 2, 8, 35, 0, 32, 2, 40, 2, 0, 65, 124, 113, 65, 4, 106, 106, 36, 0, 32, 2, 65, 20, 106,
        34, 1, 65, 0, 32, 0, 252, 11, 0, 32, 1, 11, 97, 1, 3, 127, 32, 0, 4, 64, 32, 0, 65, 20,
        107, 34, 1, 40, 2, 4, 65, 3, 113, 65, 3, 70, 4, 64, 65, 176, 11, 65, 224, 8, 65, 210, 2,
        65, 7, 16, 0, 0, 11, 32, 1, 16, 3, 35, 4, 34, 3, 40, 2, 8, 33, 2, 32, 1, 32, 3, 65, 3, 114,
        54, 2, 4, 32, 1, 32, 2, 54, 2, 8, 32, 2, 32, 1, 32, 2, 40, 2, 4, 65, 3, 113, 114, 54, 2, 4,
        32, 3, 32, 1, 54, 2, 8, 11, 32, 0, 11, 110, 1, 2, 127, 32, 0, 69, 4, 64, 15, 11, 32, 0, 65,
        20, 107, 34, 1, 40, 2, 4, 65, 3, 113, 65, 3, 71, 4, 64, 65, 240, 11, 65, 224, 8, 65, 224,
        2, 65, 5, 16, 0, 0, 11, 35, 2, 65, 1, 70, 4, 64, 32, 1, 16, 4, 5, 32, 1, 16, 3, 35, 8, 34,
        0, 40, 2, 8, 33, 2, 32, 1, 32, 0, 35, 7, 114, 54, 2, 4, 32, 1, 32, 2, 54, 2, 8, 32, 2, 32,
        1, 32, 2, 40, 2, 4, 65, 3, 113, 114, 54, 2, 4, 32, 0, 32, 1, 54, 2, 8, 11, 11, 57, 0, 35,
        2, 65, 0, 74, 4, 64, 3, 64, 35, 2, 4, 64, 16, 9, 26, 12, 1, 11, 11, 11, 16, 9, 26, 3, 64,
        35, 2, 4, 64, 16, 9, 26, 12, 1, 11, 11, 35, 0, 173, 66, 200, 1, 126, 66, 228, 0, 128, 167,
        65, 128, 8, 106, 36, 1, 11, 51, 0, 2, 64, 2, 64, 2, 64, 2, 64, 2, 64, 32, 0, 65, 8, 107,
        40, 2, 0, 14, 4, 0, 1, 2, 3, 4, 11, 15, 11, 15, 11, 15, 11, 32, 0, 40, 2, 0, 34, 0, 4, 64,
        32, 0, 16, 17, 11, 15, 11, 0, 11, 86, 0, 63, 0, 65, 16, 116, 65, 180, 140, 2, 107, 65, 1,
        118, 36, 1, 65, 148, 9, 65, 144, 9, 54, 2, 0, 65, 152, 9, 65, 144, 9, 54, 2, 0, 65, 144, 9,
        36, 4, 65, 180, 9, 65, 176, 9, 54, 2, 0, 65, 184, 9, 65, 176, 9, 54, 2, 0, 65, 176, 9, 36,
        6, 65, 196, 10, 65, 192, 10, 54, 2, 0, 65, 200, 10, 65, 192, 10, 54, 2, 0, 65, 192, 10, 36,
        8, 11, 32, 0, 35, 7, 32, 0, 65, 20, 107, 34, 0, 40, 2, 4, 65, 3, 113, 70, 4, 64, 32, 0, 16,
        4, 35, 3, 65, 1, 106, 36, 3, 11, 11, 11, 144, 3, 15, 0, 65, 140, 8, 11, 1, 60, 0, 65, 152,
        8, 11, 47, 2, 0, 0, 0, 40, 0, 0, 0, 65, 0, 108, 0, 108, 0, 111, 0, 99, 0, 97, 0, 116, 0,
        105, 0, 111, 0, 110, 0, 32, 0, 116, 0, 111, 0, 111, 0, 32, 0, 108, 0, 97, 0, 114, 0, 103,
        0, 101, 0, 65, 204, 8, 11, 1, 60, 0, 65, 216, 8, 11, 39, 2, 0, 0, 0, 32, 0, 0, 0, 126, 0,
        108, 0, 105, 0, 98, 0, 47, 0, 114, 0, 116, 0, 47, 0, 105, 0, 116, 0, 99, 0, 109, 0, 115, 0,
        46, 0, 116, 0, 115, 0, 65, 204, 9, 11, 1, 60, 0, 65, 216, 9, 11, 43, 2, 0, 0, 0, 36, 0, 0,
        0, 73, 0, 110, 0, 100, 0, 101, 0, 120, 0, 32, 0, 111, 0, 117, 0, 116, 0, 32, 0, 111, 0,
        102, 0, 32, 0, 114, 0, 97, 0, 110, 0, 103, 0, 101, 0, 65, 140, 10, 11, 1, 44, 0, 65, 152,
        10, 11, 27, 2, 0, 0, 0, 20, 0, 0, 0, 126, 0, 108, 0, 105, 0, 98, 0, 47, 0, 114, 0, 116, 0,
        46, 0, 116, 0, 115, 0, 65, 220, 10, 11, 1, 60, 0, 65, 232, 10, 11, 37, 2, 0, 0, 0, 30, 0,
        0, 0, 126, 0, 108, 0, 105, 0, 98, 0, 47, 0, 114, 0, 116, 0, 47, 0, 116, 0, 108, 0, 115, 0,
        102, 0, 46, 0, 116, 0, 115, 0, 65, 156, 11, 11, 1, 60, 0, 65, 168, 11, 11, 49, 2, 0, 0, 0,
        42, 0, 0, 0, 79, 0, 98, 0, 106, 0, 101, 0, 99, 0, 116, 0, 32, 0, 97, 0, 108, 0, 114, 0,
        101, 0, 97, 0, 100, 0, 121, 0, 32, 0, 112, 0, 105, 0, 110, 0, 110, 0, 101, 0, 100, 0, 65,
        220, 11, 11, 1, 60, 0, 65, 232, 11, 11, 47, 2, 0, 0, 0, 40, 0, 0, 0, 79, 0, 98, 0, 106, 0,
        101, 0, 99, 0, 116, 0, 32, 0, 105, 0, 115, 0, 32, 0, 110, 0, 111, 0, 116, 0, 32, 0, 112, 0,
        105, 0, 110, 0, 110, 0, 101, 0, 100, 0, 65, 160, 12, 11, 13, 4, 0, 0, 0, 32, 0, 0, 0, 32,
        0, 0, 0, 32, 0, 36, 16, 115, 111, 117, 114, 99, 101, 77, 97, 112, 112, 105, 110, 103, 85,
        82, 76, 18, 46, 47, 114, 101, 108, 101, 97, 115, 101, 46, 119, 97, 115, 109, 46, 109, 97,
        112,
    ];
}
