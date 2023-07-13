use std::collections::HashMap;

use mamoru_core::{Daemon, DaemonParameters, DataError};

use crate::assembly_script::AssemblyScriptModule;

pub mod assembly_script;
pub mod test_blockchain_data;

pub fn test_daemon(module: &AssemblyScriptModule) -> Daemon {
    try_test_daemon(module, DaemonParameters::new()).expect("Failed to create daemon.")
}

pub fn test_daemon_with_parameters(
    module: &AssemblyScriptModule,
    parameters: DaemonParameters,
) -> Daemon {
    try_test_daemon(module, parameters).expect("Failed to create daemon.")
}

pub fn try_test_daemon(
    module: &AssemblyScriptModule,
    parameters: DaemonParameters,
) -> Result<Daemon, DataError> {
    Daemon::new_assembly_script(
        "dummy".to_string(),
        module.bytes(),
        parameters,
        HashMap::new(),
    )
}
