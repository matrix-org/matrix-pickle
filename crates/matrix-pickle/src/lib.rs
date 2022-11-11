// Copyright 2021, 2022 Damir Jelić
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(
    clippy::mem_forget,
    clippy::unwrap_used,
    dead_code,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate self as matrix_pickle;

mod decode;
mod encode;
mod error;

const MAX_ARRAY_LENGTH: usize = u16::MAX as usize;

#[cfg(feature = "derive")]
pub use matrix_pickle_derive::*;

pub use decode::*;
pub use encode::*;
pub use error::*;

#[cfg(test)]
mod test {
    use proptest::prelude::*;

    use super::*;

    macro_rules! encode_cycle {
        ($value:expr => $type:ty) => {
            let value = $value;

            let encoded = value
                .encode_to_vec()
                .expect("We can always encode into to a Vec");
            let decoded = <$type>::decode_from_slice(&encoded)
                .expect("Decoding a freshly encoded value always works");

            assert_eq!(
                value, decoded,
                "The original value and the decoded value are not the same"
            );
        };
    }

    #[test]
    fn encode_cycle() {
        encode_cycle!(10u8 => u8);
        encode_cycle!(10u32 => u32);
        encode_cycle!(10usize => usize);
        encode_cycle!(true => bool);
        encode_cycle!(false => bool);
        encode_cycle!(vec![1, 2, 3, 4] => Vec<u8>);
    }

    proptest! {
        #[test]
        fn encode_cycle_u8(a in 0..u8::MAX) {
            encode_cycle!(a => u8);
        }

        #[test]
        fn encode_cycle_u32(a in 0..u32::MAX) {
            encode_cycle!(a => u32);
        }

        #[test]
        fn encode_cycle_usize(a in 0..u32::MAX) {
            let a = a as usize;
            encode_cycle!(a => usize);
        }

        fn encode_cycle_vec(bytes in prop::collection::vec(any::<u8>(), 0..1000)) {
            encode_cycle!(bytes => Vec<u8>);
        }
    }

    #[test]
    #[cfg(feature = "derive")]
    fn derive() {
        #[derive(Clone, Encode, Decode, PartialEq, Debug)]
        struct Foo {
            thing: [u8; 32],
            #[secret]
            another: Box<[u8; 64]>,
        }

        let foo = Foo {
            thing: [1u8; 32],
            another: Box::new([2u8; 64]),
        };

        encode_cycle!(foo.clone() => Foo);

        #[derive(Clone, Encode, Decode, PartialEq, Debug)]
        struct Bar([u8; 32]);

        let bar = Bar([1u8; 32]);
        encode_cycle!(bar.clone() => Bar);

        #[derive(Encode, Decode, PartialEq, Debug)]
        enum Something {
            Foo(Foo),
            Bar(Bar),
        }

        let something = Something::Foo(foo);
        encode_cycle!(something => Something);

        let something = Something::Bar(bar);
        encode_cycle!(something => Something);
    }
}
