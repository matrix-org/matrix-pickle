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

use std::io::{Cursor, Write};

use crate::{EncodeError, MAX_ARRAY_LENGTH};

pub trait Encode {
    fn encode(&self, writer: &mut impl Write) -> Result<usize, EncodeError>;

    fn encode_to_vec(&self) -> Result<Vec<u8>, EncodeError> {
        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);

        self.encode(&mut cursor)?;

        Ok(cursor.into_inner())
    }
}

impl Encode for u8 {
    fn encode(&self, writer: &mut impl Write) -> Result<usize, EncodeError> {
        Ok(writer.write(&[*self])?)
    }
}

impl Encode for bool {
    fn encode(&self, writer: &mut impl Write) -> Result<usize, EncodeError> {
        (*self as u8).encode(writer)
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode(&self, writer: &mut impl Write) -> Result<usize, EncodeError> {
        writer.write_all(self)?;

        Ok(self.len() * 8)
    }
}

impl Encode for u32 {
    fn encode(&self, writer: &mut impl Write) -> Result<usize, EncodeError> {
        let bytes = self.to_be_bytes();
        bytes.encode(writer)
    }
}

impl Encode for usize {
    fn encode(&self, writer: &mut impl Write) -> Result<usize, EncodeError> {
        let value = u32::try_from(*self).map_err(|_| EncodeError::OutsideU32Range(*self))?;

        value.encode(writer)
    }
}

impl<T: Encode> Encode for [T] {
    fn encode(&self, writer: &mut impl Write) -> Result<usize, EncodeError> {
        let length = self.len();

        if length > MAX_ARRAY_LENGTH {
            Err(EncodeError::ArrayTooBig(length))
        } else {
            let mut ret = length.encode(writer)?;

            for value in self {
                ret += value.encode(writer)?;
            }

            Ok(ret)
        }
    }
}
