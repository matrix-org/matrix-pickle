// Copyright 2022 Damir Jelić
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
use crate::{EncodeError, MAX_ARRAY_LENGTH};
use alloc::vec::Vec;
use bytes::{buf::BufMut, BytesMut};
use core::mem::size_of;

/// A trait for encoding values into the `matrix-pickle` binary format.
pub trait Encode {
    /// Try to encode and write a value to the given writer.
    fn encode(&self, buf: &mut impl BufMut) -> Result<usize, EncodeError>;

    /// Try to encode a value into a new `Vec`.
    fn encode_to_vec(&self) -> Result<Vec<u8>, EncodeError> {
        let mut buffer = BytesMut::new();
        self.encode(&mut buffer)?;

        Ok(buffer.into())
    }
}

impl Encode for u8 {
    fn encode(&self, buf: &mut impl BufMut) -> Result<usize, EncodeError> {
        buf.put_u8(*self);
        Ok(size_of::<Self>())
    }
}

impl Encode for bool {
    fn encode(&self, buf: &mut impl BufMut) -> Result<usize, EncodeError> {
        (*self as u8).encode(buf)
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode(&self, buf: &mut impl BufMut) -> Result<usize, EncodeError> {
        buf.put(&self[..]);

        Ok(size_of::<Self>())
    }
}

impl Encode for u32 {
    fn encode(&self, buf: &mut impl BufMut) -> Result<usize, EncodeError> {
        buf.put_u32(*self);

        Ok(size_of::<Self>())
    }
}

impl Encode for usize {
    fn encode(&self, buf: &mut impl BufMut) -> Result<usize, EncodeError> {
        let value = u32::try_from(*self).map_err(|_| EncodeError::OutsideU32Range(*self))?;

        value.encode(buf)
    }
}

impl<T: Encode> Encode for [T] {
    fn encode(&self, buf: &mut impl BufMut) -> Result<usize, EncodeError> {
        let length = self.len();

        if length > MAX_ARRAY_LENGTH {
            Err(EncodeError::ArrayTooBig(length))
        } else {
            let mut ret = length.encode(buf)?;

            for value in self {
                ret += value.encode(buf)?;
            }

            Ok(ret)
        }
    }
}
