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

use thiserror::Error;

/// Error type describing failure modes for libolm pickle decoding.
#[derive(Debug, Error)]
pub enum DecodeError {
    /// There was an error while reading from the source of the libolm, usually
    /// not enough data was provided.
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// The encoded usize doesn't fit into the usize of the architecture that is
    /// decoding.
    #[error(
        "The decoded value {0} does not fit into the usize type of this \
         architecture"
    )]
    OutsideUsizeRange(u64),
    /// An array in the pickle has too many elements.
    #[error("An array has too many elements: {0}")]
    ArrayTooBig(usize),
    /// TODO
    #[error("TODO {0}")]
    UnknownEnumVariant(u8),
}

/// Error type describing failure modes for libolm pickle decoding.
#[derive(Debug, Error)]
pub enum EncodeError {
    /// There was an error while writing to the buffer.
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// The usize value that should be encoded doesn't fit into the u32 range of
    /// values.
    #[error("The usize value {0} does not fit into the u32 range of values.")]
    OutsideU32Range(usize),
    /// An array in the pickle has too many elements.
    #[error("An array has too many elements: {0}")]
    ArrayTooBig(usize),
}
