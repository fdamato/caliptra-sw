/*++

Licensed under the Apache-2.0 license.

File Name:

    helpers.rs

Abstract:

    File contains helper functions

--*/
#![allow(unused)]

use std::ops::{Index, IndexMut, RangeFrom};

pub trait U32Array: AsRef<[u32]> {
    const BYTE_LEN: usize;
    type ByteArray: AsMut<[u8]>;
    // Required because Default isn't implemented for [u8; 64]
    fn default_result() -> Self::ByteArray;
}
pub trait U32ArrayBytes: AsRef<[u8]> {
    const WORD_LEN: usize;
    type WordArray: AsMut<[u32]> + Default;
}
macro_rules! u32_array_impl {
    ($word_len:literal) => {
        impl U32Array for [u32; $word_len] {
            const BYTE_LEN: usize = $word_len * 4;
            type ByteArray = [u8; $word_len * 4];
            fn default_result() -> Self::ByteArray {
                [0u8; $word_len * 4]
            }
        }
        impl U32ArrayBytes for [u8; $word_len * 4] {
            const WORD_LEN: usize = $word_len;
            type WordArray = [u32; $word_len];
        }
    };
}
u32_array_impl!(0);
u32_array_impl!(1);
u32_array_impl!(2);
u32_array_impl!(3);
u32_array_impl!(4);
u32_array_impl!(5);
u32_array_impl!(6);
u32_array_impl!(7);
u32_array_impl!(8);
u32_array_impl!(9);
u32_array_impl!(10);
u32_array_impl!(11);
u32_array_impl!(12);
u32_array_impl!(13);
u32_array_impl!(14);
u32_array_impl!(15);
u32_array_impl!(16);

pub fn words_from_bytes_le<A: U32ArrayBytes>(arr: &A) -> A::WordArray {
    let mut result = A::WordArray::default();
    for i in 0..result.as_mut().len() {
        result.as_mut()[i] = u32::from_le_bytes(arr.as_ref()[i * 4..][..4].try_into().unwrap())
    }
    result
}

pub fn bytes_from_words_le<A: U32Array>(arr: &A) -> A::ByteArray {
    let mut result = A::default_result();
    for i in 0..arr.as_ref().len() {
        result.as_mut()[i * 4..][..4].copy_from_slice(&arr.as_ref()[i].to_le_bytes());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_words_from_bytes_le() {
        assert_eq!(
            [
                0xeb457996, 0x70cdc537, 0x95684f6f, 0xde320a3e, 0xbe5a8d48, 0xd25c3fe1, 0x47051dda,
                0x2fcb1d61, 0x5abaabff, 0xd041c30e, 0x1a18b955, 0x117495d4,
            ],
            words_from_bytes_le(&[
                0x96, 0x79, 0x45, 0xeb, 0x37, 0xc5, 0xcd, 0x70, 0x6f, 0x4f, 0x68, 0x95, 0x3e, 0x0a,
                0x32, 0xde, 0x48, 0x8d, 0x5a, 0xbe, 0xe1, 0x3f, 0x5c, 0xd2, 0xda, 0x1d, 0x05, 0x47,
                0x61, 0x1d, 0xcb, 0x2f, 0xff, 0xab, 0xba, 0x5a, 0x0e, 0xc3, 0x41, 0xd0, 0x55, 0xb9,
                0x18, 0x1a, 0xd4, 0x95, 0x74, 0x11
            ])
        );
        assert_eq!(
            [
                0x6d533aa5, 0xb8066955, 0xb1d55284, 0xe44aade8, 0x279ad4d2, 0xde9de661, 0x13fba077,
                0x76a6efd,
            ],
            words_from_bytes_le(&[
                0xa5, 0x3a, 0x53, 0x6d, 0x55, 0x69, 0x06, 0xb8, 0x84, 0x52, 0xd5, 0xb1, 0xe8, 0xad,
                0x4a, 0xe4, 0xd2, 0xd4, 0x9a, 0x27, 0x61, 0xe6, 0x9d, 0xde, 0x77, 0xa0, 0xfb, 0x13,
                0xfd, 0x6e, 0x6a, 0x7
            ])
        );
    }

    #[test]
    pub fn test_bytes_from_words_le() {
        assert_eq!(
            [
                0x96, 0x79, 0x45, 0xeb, 0x37, 0xc5, 0xcd, 0x70, 0x6f, 0x4f, 0x68, 0x95, 0x3e, 0x0a,
                0x32, 0xde, 0x48, 0x8d, 0x5a, 0xbe, 0xe1, 0x3f, 0x5c, 0xd2, 0xda, 0x1d, 0x05, 0x47,
                0x61, 0x1d, 0xcb, 0x2f, 0xff, 0xab, 0xba, 0x5a, 0x0e, 0xc3, 0x41, 0xd0, 0x55, 0xb9,
                0x18, 0x1a, 0xd4, 0x95, 0x74, 0x11
            ],
            bytes_from_words_le(&[
                0xeb457996, 0x70cdc537, 0x95684f6f, 0xde320a3e, 0xbe5a8d48, 0xd25c3fe1, 0x47051dda,
                0x2fcb1d61, 0x5abaabff, 0xd041c30e, 0x1a18b955, 0x117495d4,
            ])
        );
        assert_eq!(
            [
                0xa5, 0x3a, 0x53, 0x6d, 0x55, 0x69, 0x06, 0xb8, 0x84, 0x52, 0xd5, 0xb1, 0xe8, 0xad,
                0x4a, 0xe4, 0xd2, 0xd4, 0x9a, 0x27, 0x61, 0xe6, 0x9d, 0xde, 0x77, 0xa0, 0xfb, 0x13,
                0xfd, 0x6e, 0x6a, 0x7
            ],
            bytes_from_words_le(&[
                0x6d533aa5, 0xb8066955, 0xb1d55284, 0xe44aade8, 0x279ad4d2, 0xde9de661, 0x13fba077,
                0x76a6efd,
            ])
        );
    }
}
