# RSEC :lock:

***Test the strength of your passwords at hypersonic speeds.*** :lock:

![GitHub CI](https://github.com/iamtheblackunicorn/rsec/actions/workflows/rust.yml/badge.svg)

## ABOUT :books:

Some time ago, I implemented my own algorithm to check the strength of a password in Dart [here](https://github.com/iamtheblackunicorn/securitycheck). This is the RTust implementation of that library.

## FEATURES :test_tube:

- Blazingly fast.
- Easy to use, no drama.

## INSTALLATION :inbox_tray:

To use ***RSec*** in your rust project, add this line to your project's `Cargo.toml`'s `[dependencies]` section:

```TOML
rsec = { git = "https://github.com/iamtheblackunicorn/rsec", version = "1.0.0" }
```

To import the library into your project's code, use this line:

```Rust
use rsec::*;
```

To find out exactly how to use the library, please check out the section below.

## USAGE :hammer:

To generate documentation, assuming you have the Rust toolchain installed, run this command inside this library's root directory:

```bash
cargo doc --target-dir .
```

To view the library's documentation, please click [here](https://blckunicorn.art/rsec).

## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *RSec :lock:* by Alexander Abraham :black_heart: a.k.a. *"The Black Unicorn" :unicorn:*
- Licensed under the MIT license.
