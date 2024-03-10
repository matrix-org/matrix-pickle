![Build Status](https://img.shields.io/github/actions/workflow/status/matrix-org/matrix-pickle/ci.yml?branch=main&style=flat-square)
[![codecov](https://img.shields.io/codecov/c/github/matrix-org/matrix-pickle/main.svg?style=flat-square)](https://codecov.io/gh/matrix-org/matrix-pickle)
[![License](https://img.shields.io/badge/License-MIT-yellowgreen.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Docs](https://img.shields.io/crates/v/matrix-pickle?color=blue&label=docs&style=flat-square)](https://docs.rs/matrix-pickle)


A simple binary encoding format used in the Matrix world.

The `matrix-pickle` binary encoding format is used in the [libolm] and
[vodozemac] cryptographic libraries.

# How to use

The simplest way to use `matrix-pickle` is using the derive macros:

```rust
use anyhow::Result;
use matrix_pickle::{Encode, Decode};

fn main() -> Result<()> {
    #[derive(Clone, Debug, Decode, Encode, PartialEq, Eq)]
    struct MyStruct {
        public_key: [u8; 32],
        data: Vec<u8>,
    }
    
    let data = MyStruct {
        public_key: [5u8; 32],
        data: vec![1, 2, 3],
    };
    
    let encoded = data.encode_to_vec()?;
    let decoded = MyStruct::decode_from_slice(&encoded)?;
    
    assert_eq!(data, decoded);

    Ok(())
}
```

# Format definition

`matrix-pickle` encodes most values without any metadata, the bytes that are
part of the struct in most cases get encoded verbatim.

The table bellow defines how common types are encoded.

|   Type    | Example value |       Encoded value        |                      Comment                     |
| :-------: | :-----------: | :------------------------: | ------------------------------------------------ |
|   `u8`    |     `255`     |           `[FF]`           | Encoded verbatim                                 |
|  `bool`   |    `true`     |           `[01]`           | Converted to an `u8` before encoding             |
| `[u8; N]` | `[1u8, 2u8]`  |         `[01, 02]`         | Encoded verbatim                                 |
|   `u32`   |     `16`      |     `[00, 00, 00, 10]`     | Encoded as a byte array in big endian form       |
|  `usize`  |     `32`      |     `[00, 00, 00, 20]`     | Converted to an `u32` before encoding            |
|  `&[T]`   | `&[3u8, 4u8]` | `[00, 00, 00, 02, 03, 04]` | The length gets encoded first, then each element |

# Derive support

The crate supports deriving `Encode` and `Decode` implementations for structs
and enums as long as the types inside them implement `Encode` and `Decode` as
well.

## Structs

The derive support for structs simply encodes each field of a struct in the order
they are defined, for example:

```rust
use matrix_pickle::{Encode, EncodeError};

struct Foo {
    first: [u8; 32],
    second: Vec<u8>,
}

impl Encode for Foo {
    fn encode(&self, buf: &mut impl bytes::buf::BufMut) -> Result<usize, EncodeError> {
        let mut ret = 0;

        // Encode the first struct field.
        ret += self.first.encode(buf)?;
        // Now encode the second struct field.
        ret += self.second.encode(buf)?;

        Ok(ret)
    }
}
```

## Enums

Enums on the other hand first encode the number of the variant as an `u8`, then
the value of the enum.

Only enums with variants that contain a single associated data value are
supported.

```rust
use matrix_pickle::{Encode, EncodeError};

enum Bar {
    First(u32),
    Second(u32),
}

impl Encode for Bar {
    fn encode(&self, buf: &mut impl bytes::buf::BufMut) -> Result<usize, EncodeError> {
        let mut ret = 0;

        match self {
            Bar::First(value) => {
                // This is our first variant, encode a 0u8 first.
                ret += 0u8.encode(buf)?;
                // Now encode the associated value.
                ret += value.encode(buf)?;
            },
            Bar::Second(value) => {
                // This is our second variant, encode a 1u8 first.
                ret += 1u8.encode(buf)?;
                // Now encode the associated value.
                ret += value.encode(buf)?;
            },
        }

        Ok(ret)
    }
}
```

## Encoding and decoding secrets

For decoding values which are meant to be secret, make sure to box the array. We
have a helper attribute that reminds you that values that are meant to be kept
secret should be boxed.

Simply annotate any struct field using the `#[secret]` attribute.

If a value that is meant to be a secret is not boxed a compiler error will be
thrown. For example, this snippet won't compile.

```rust,compile_fail
use matrix_pickle::{Encode, Decode};

#[derive(Encode, Decode)]
struct Key {
    #[secret]
    private: [u8; 32],
    public: [u8; 32],
}
```

This example on the other hand compiles.

```rust
use matrix_pickle::{Encode, Decode};

#[derive(Encode, Decode)]
struct Key {
    #[secret]
    private: Box<[u8; 32]>,
    public: [u8; 32],
}
```


# Comparison to bincode

The binary format is similar to what the [bincode] crate provides with the
following config:

```rust,compile_fail
let config = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding()
    .skip_fixed_array_length();
```

The two major differences to the format are:

* `bincode` uses `u64` to encode slice lengths
* `matrix-pickle` uses `u32` to encode slice lengths

Other differences are:

* No support to configure the encoding format, if you need to tweak the
  format, use bincode.
* No unsafe code. Optimized for simplicity, not for pure performance

[libolm]: https://gitlab.matrix.org/matrix-org/olm/
[vodozemac]: https://github.com/matrix-org/vodozemac/
[bincode]: https://github.com/bincode-org/bincode/
