mod number;
mod macros;

use thiserror::Error;

// TODO: Inline all unpack functions?

pub trait UnpackLength<'a>: Sized {
    type Error;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error>;

    fn peek(buf: &'a [u8], len: usize) -> Result<Self, Self::Error> {
        Self::unpack(&mut &buf[..], len)
    }
}

pub trait Unpack: Sized {
    type Error;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error>;

    fn peek(buf: &[u8]) -> Result<Self, Self::Error> {
        Self::unpack(&mut &buf[..])
    }
}

#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum UnpackError {
    #[error("not enough bytes to unpack (expected {expected}, found {found})")]
    NotEnoughBytes { expected: usize, found: usize },
    #[error("could not decode UTF-8 str")]
    Utf8Error(#[from] std::str::Utf8Error),
}

impl<'a> UnpackLength<'a> for &'a [u8] {
    type Error = UnpackError;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        if buf.len() < len {
            Err(UnpackError::NotEnoughBytes {
                expected: len,
                found: buf.len(),
            })
        } else {
            let (len_bytes, rest) = buf.split_at(len);
            *buf = rest;
            Ok(len_bytes)
        }
    }
}

impl<'a> UnpackLength<'a> for &'a str {
    type Error = UnpackError;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        let bytes = UnpackLength::unpack(buf, len)?;
        Ok(std::str::from_utf8(bytes)?)
    }
}

impl<'a> UnpackLength<'a> for Vec<u8> {
    type Error = UnpackError;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        let bytes: &[u8] = UnpackLength::unpack(buf, len)?;
        Ok(bytes.to_vec())
    }
}

impl<'a> UnpackLength<'a> for String {
    type Error = UnpackError;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        let str: &'a str = UnpackLength::unpack(buf, len)?;
        Ok(str.to_string())
    }
}

impl<'a, U: Unpack> UnpackLength<'a> for Vec<U> {
    type Error = U::Error;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        let mut items = Vec::with_capacity(len);

        for _ in 0..len {
            items.push(U::unpack(buf)?);
        }

        Ok(items)
    }
}

impl<const N: usize> Unpack for [u8; N] {
    type Error = UnpackError;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8] = UnpackLength::unpack(buf, N)?;
        Ok(bytes.try_into().unwrap())
    }
}

impl<const N: usize, T: Unpack> Unpack for [T; N] {
    type Error = T::Error;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let items: Vec<T> = UnpackLength::unpack(buf, N)?;
        Ok(items.try_into().ok().unwrap())
    }
}