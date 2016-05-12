## Dummy rustwlc

**Currently matching rustwlc v0.3.2**

This project exists so one can compile [rust-wlc](https://github.com/Immington-Industries/rust-wlc) without needing C library references.

It is used in [way-cooler](https://github.com/Immington-Industries/way-cooler)'s Travis build script.

### Usage

All methods with a return type have their implementation replaced with `unimplemented!()`, calling their code will cause yours to panic.

The methods in `callbacks` will run without side effects and `rustwlc::init` returns a function that simply prints a message to the console.

Effectively, dropping in `dummy-rustwlc` and running your program _should_ cause your program to start, print the message, and exit successfully.

### Build Script

To replace a project's `rustwlc` depenency, one can run this script from the project root to swap dependencies in `Cargo.toml`.

```bash
sed 's/rustwlc = .*/rustwlc = { git = "https:\/\/github.com\/SnirkImmington\/dummy-rustwlc.git" }/g' Cargo.toml -i
```
