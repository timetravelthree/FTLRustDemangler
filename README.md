# FTL Rust Demangler 🦀

[![Rust Community](https://img.shields.io/badge/Rust_Community%20-Join_us-brightgreen?style=plastic&logo=rust)](https://www.rust-lang.org/community)


FTL Rust Demangler is a command-line tool for demangling symbol names that are mangled with the Rust convention. It takes a mangled symbol name as input and returns the demangled name.

# Build
To build Rust Demangler, you'll need to have Rust installed on your system. Once you have Rust installed, you can use Cargo to build it

```sh
cargo build --release
```

# Usage

To use Rust Demangler, simply run the tool with a mangled symbol name as an argument:

```sh
rs-dml first_symbol [second_symbol [...]]
```

# Example


```sh
rs-dml '_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h03d70cf32f8d87a3E'
```
This will output the demangled symbol name:

```sh
<core::fmt::Arguments as core::fmt::Display>::fmt
```

# Contributing
Contributions are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request.

# Todo

- [ ] add support for v0 symbols
- [ ] multiple builds for different os
- [ ] make it also a usable library
- [ ] simple unit testing
- [ ] add crate


# License
FTL Rust Demangler is licensed under the GPL-2.0 license. See the LICENSE file for details.
