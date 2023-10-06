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
use std::io::{Cursor, Read};

use crate::{DecodeError, MAX_ARRAY_LENGTH};

/// A trait for decoding values that were encoded using the `matrix-pickle` binary format.
pub trait Decode {
    /// Try to read and decode a value from the given reader.
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError>
    where
        Self: Sized;

    /// Try to read and decode a value from the given byte slice.
    fn decode_from_slice(buffer: &[u8]) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut cursor = Cursor::new(buffer);
        Self::decode(&mut cursor)
    }
}

impl Decode for u8 {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        let mut buffer = [0u8; 1];

        reader.read_exact(&mut buffer)?;

        Ok(buffer[0])
    }
}

impl Decode for bool {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        let value = u8::decode(reader)?;

        Ok(value != 0)
    }
}

impl Decode for u32 {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;

        Ok(u32::from_be_bytes(buffer))
    }
}

impl Decode for usize {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        let size = u32::decode(reader)?;

        size.try_into()
            .map_err(|_| DecodeError::OutsideUsizeRange(size as u64))
    }
}

impl<const N: usize> Decode for [u8; N] {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        let mut buffer = [0u8; N];
        reader.read_exact(&mut buffer)?;

        Ok(buffer)
    }
}

impl<const N: usize> Decode for Box<[u8; N]> {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        let mut buffer = Box::new([0u8; N]);
        reader.read_exact(buffer.as_mut_slice())?;

        Ok(buffer)
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(reader: &mut impl Read) -> Result<Self, DecodeError> {
        let length = usize::decode(reader)?;

        if length > MAX_ARRAY_LENGTH {
            Err(DecodeError::ArrayTooBig(length))
        } else {
            let mut buffer = Vec::with_capacity(length);

            for _ in 0..length {
                let element = T::decode(reader)?;
                buffer.push(element);
            }

            Ok(buffer)
        }
    }
}
