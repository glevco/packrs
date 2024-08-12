use thiserror::Error;

pub trait Unpack<'a>: Sized {
    type Error;

    fn unpack(buf: &mut &'a [u8]) -> Result<Self, Self::Error>;

    fn peek(buf: &'a [u8]) -> Result<Self, Self::Error> {
        Self::unpack(&mut &buf[..])
    }
}

pub trait UnpackLength<'a>: Sized {
    type Error;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error>;

    fn peek(buf: &'a [u8], len: usize) -> Result<Self, Self::Error> {
        Self::unpack(&mut &buf[..], len)
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
        let (len_bytes, rest) = buf.split_at_checked(len).ok_or(UnpackError::NotEnoughBytes {
            expected: len,
            found: buf.len(),
        })?;

        *buf = rest;
        Ok(len_bytes)
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

impl<'a, U: Unpack<'a>> UnpackLength<'a> for Vec<U> {
    type Error = U::Error;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        let mut items = Vec::with_capacity(len);

        for _ in 0..len {
            items.push(U::unpack(buf)?);
        }

        Ok(items)
    }
}

impl<const N: usize> Unpack<'_> for [u8; N] {
    type Error = UnpackError;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let bytes: &[u8] = UnpackLength::unpack(buf, N)?;
        Ok(bytes.try_into().unwrap())
    }
}

impl<'a, const N: usize, T: Unpack<'a>> Unpack<'a> for [T; N] {
    type Error = T::Error;

    fn unpack(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
        let items: Vec<T> = UnpackLength::unpack(buf, N)?;
        Ok(items.try_into().ok().unwrap())
    }
}
