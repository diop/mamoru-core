use std::{
    ffi::OsStr,
    fs,
    path::Path,
    process::{Command, Stdio},
};

use tempfile::TempDir;

pub const AS_SDK_PATH: &str = concat!(
    "file:",
    env!("CARGO_MANIFEST_DIR"),
    "/../sdk-as/mamoru-sdk-as"
);

pub const AS_EVM_SDK_PATH: &str = concat!(
    "file:",
    env!("CARGO_MANIFEST_DIR"),
    "/../sdk-as/mamoru-evm-sdk-as"
);

pub const AS_SUI_SDK_PATH: &str = concat!(
    "file:",
    env!("CARGO_MANIFEST_DIR"),
    "/../sdk-as/mamoru-sui-sdk-as"
);

pub const AS_APTOS_SDK_PATH: &str = concat!(
    "file:",
    env!("CARGO_MANIFEST_DIR"),
    "/../sdk-as/mamoru-aptos-sdk-as"
);

/// Setups AssemblyScript project for tests.
pub struct AssemblyScriptModule {
    _dir_guard: TempDir,
    wasm: Vec<u8>,
}

impl AssemblyScriptModule {
    pub fn new(code: &str) -> Self {
        let code_dir = Self::init_app_template(code);
        let wasm = Self::compile(code_dir.path());

        Self {
            _dir_guard: code_dir,
            wasm,
        }
    }

    pub fn with_deps(code: &str, deps: &[&str]) -> Self {
        let code_dir = Self::init_app_template(code);
        let path = code_dir.path();

        for dep in deps {
            run_cmd(path, ["npm", "install", "--save", dep]);
        }

        let wasm = Self::compile(path);

        Self {
            _dir_guard: code_dir,
            wasm,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        self.wasm.as_ref()
    }

    fn init_app_template(code: &str) -> TempDir {
        let code_dir = tempfile::Builder::new()
            .prefix("mamoru-test")
            .tempdir()
            .expect("Failed to create a temp dir.");

        let path = code_dir.path();

        run_cmd(path, ["npm", "init", "--yes"]);
        run_cmd(
            path,
            ["npm", "install", "--save-dev", "assemblyscript@0.27.5"],
        );
        run_cmd(path, ["npx", "asinit", ".", "--yes"]);

        fs::write(path.join("./assembly/index.ts"), code).expect("Failed to write index.ts");

        code_dir
    }

    fn compile(path: &Path) -> Vec<u8> {
        run_cmd(
            path,
            [
                "npx",
                "asc",
                "assembly/index.ts",
                "--target",
                "debug",
                "--exportRuntime",
            ],
        );

        fs::read(path.join("./build/debug.wasm")).expect("Failed to read compiled WASM code.")
    }
}

fn run_cmd<I, S>(path: &Path, cmd: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = cmd.into_iter();

    let command = cmd.next().expect("At least command name passed.");
    let args: Vec<_> = cmd.collect();

    let output = Command::new(command)
        .current_dir(path)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to run command.");

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));

        panic!("Command failed.");
    }
}
