## C ABI compatible layer for `query-validator`

The layer enables developers to use `mamoru-core` with 
any programming language that can call C libraries.


## Usage

Add this crate as a dependency to your project and compile:

```toml
query-validator-c = {git = "ssh://git@github.com/Mamoru-Foundation/mamoru-core.git", branch = "main"}
```

It is recommended to setup binary size optimization:
```toml
[profile.release]
opt-level = "z"
strip = "debuginfo"
lto = true
```

---

## Generate headers

```shell
make headers
```

Headers are available at crate's root folder: `headers.h`.
Copy the file to the target project to use.
