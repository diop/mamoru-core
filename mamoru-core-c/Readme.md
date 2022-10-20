## C ABI compatible layer for `mamoru-core`

The layer enables developers to use `mamoru-core` with 
any programming language that can call C libraries.


## Usage

Add this crate as a dependency to your project and compile:

```toml
mamoru-core-c = {git = "ssh://git@github.com/Mamoru-Foundation/mamoru-core.git", branch = "main"}
```

It is recommended to setup binary size optimization:
```toml
[profile.release]
opt-level = 3
strip = "debuginfo"
lto = true
```

Also, use static linking to prevent `glibc` issues:
```toml
[lib]
crate-type = ["staticlib"]
```

---

## Generate headers

```shell
make headers
```

Headers are available at crate's root folder: `headers.h`.
Copy the file to the target project to use.


## Current limitations

All FFI exports should be in `lib.rs` file.
