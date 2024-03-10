// Copyright 2021 Damir Jelić
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
use crate::{DecodeError, MAX_ARRAY_LENGTH};
use alloc::{boxed::Box, vec::Vec};
use bytes::buf::Buf;

/// A trait for decoding values that were encoded using the `matrix-pickle` binary format.
pub trait Decode {
    /// Try to read and decode a value from the given reader.
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError>
    where
        Self: Sized;

    /// Try to read and decode a value from the given byte slice.
    fn decode_from_slice(buf: &[u8]) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut b = buf;
        Self::decode(&mut b)
    }
}

impl Decode for u8 {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        Ok(buf.get_u8())
    }
}

impl Decode for bool {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let value = u8::decode(buf)?;

        Ok(value != 0)
    }
}

impl Decode for u32 {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        Ok(buf.get_u32())
    }
}

impl Decode for usize {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let size = u32::decode(buf)?;

        size.try_into()
            .map_err(|_| DecodeError::OutsideUsizeRange(size as u64))
    }
}

impl<const N: usize> Decode for [u8; N] {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let mut dest = [0u8; N];
        if buf.remaining() < N {
            return Err(DecodeError::InsufficientData);
        }
        buf.copy_to_slice(&mut dest);

        Ok(dest)
    }
}

impl<const N: usize> Decode for Box<[u8; N]> {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let mut dest = Box::new([0u8; N]);
        buf.copy_to_slice(dest.as_mut_slice());

        Ok(dest)
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let length = usize::decode(buf)?;

        if length > MAX_ARRAY_LENGTH {
            Err(DecodeError::ArrayTooBig(length))
        } else {
            let mut dest = Vec::with_capacity(length);

            for _ in 0..length {
                let element = T::decode(buf)?;
                dest.push(element);
            }

            Ok(dest)
        }
    }
}
