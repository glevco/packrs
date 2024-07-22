use std::ops::Deref;
use num_traits::FromBytes;
use crate::{Unpack, UnpackError};

#[derive(Debug)]
pub struct BigEndian<T: FromBytes>(T);

#[derive(Debug)]
pub struct LittleEndian<T: FromBytes>(T);

impl<T: FromBytes> Deref for BigEndian<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: FromBytes> Deref for LittleEndian<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl<const N: usize, T: FromBytes<Bytes = [u8; N]>> Unpack for BigEndian<T> {
    type Error = UnpackError;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let bytes = Unpack::unpack(buf)?;
        let num = FromBytes::from_be_bytes(&bytes);
        Ok(BigEndian(num))
    }
}

impl<const N: usize, T: FromBytes<Bytes = [u8; N]>> Unpack for LittleEndian<T> {
    type Error = UnpackError;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let bytes = Unpack::unpack(buf)?;
        let num = FromBytes::from_le_bytes(&bytes);
        Ok(LittleEndian(num))
    }
}