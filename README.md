# Calx

[![Build & Test](https://github.com/rudio-rs/calx/actions/workflows/test.yml/badge.svg)](https://github.com/rudio-rs/calx/actions/workflows/test.yml)

## TODO

- Return an error when `StringRef::to_utf8` fails
- Consider using `String::from_utf8` instead of `String::from_utf8_lossy` for some APIs and return a custom error that includes `FromUtf8Error`
