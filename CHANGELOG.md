# Changelog

All notable changes to this project will be documented in this file.

## [0.2.2] - 2025-11-04

### 🐛 Bug Fixes

- Fixed license discrepancies between the source code and the LICENSE file and
  license definitions in the Cargo.toml file

## [0.2.1] - 2024-09-10

### 🚜 Refactor

- Switch to the `proc-macro-error2` crate as the `proc-macro-error` crate has
  been marked as unmaintained.

### 🧪 Testing

- Add a test that we properly return an error when an array reaches MAX_ARRAY_LENGTH ([#7](https://github.com/matrix-org/matrix-pickle/pull/7))

## [0.2.0] - 2024-03-25

### 🐛 Bug Fixes

- Return the correct number of bytes written for u8-arrays ([#6](https://github.com/matrix-org/matrix-pickle/pull/6))

## [0.1.1] - 2023-10-06

### 🐛 Bug Fixes

- Correctly enable syn features the matrix-pickle-derive crate is using,
  fixing compilation issues ([#3](https://github.com/matrix-org/matrix-pickle/pull/3))
