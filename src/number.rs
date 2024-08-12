use num_traits::{FromBytes, ToBytes};
use std::ops::Deref;
use crate::pack::Pack;
use crate::unpack::{Unpack, UnpackError};

#[derive(Debug)]
pub struct BigEndian<T: FromBytes + ToBytes>(T);

#[derive(Debug)]
pub struct LittleEndian<T: FromBytes + ToBytes>(T);

impl<T: FromBytes + ToBytes> Deref for BigEndian<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: FromBytes + ToBytes> Deref for LittleEndian<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize, T> Unpack<'_> for BigEndian<T>
where
    T: ToBytes + FromBytes<Bytes = [u8; N]>,
{
    type Error = UnpackError;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let bytes = Unpack::unpack(buf)?;
        let num = FromBytes::from_be_bytes(&bytes);
        Ok(BigEndian(num))
    }
}

impl<const N: usize, T> Unpack<'_> for LittleEndian<T>
where
    T: ToBytes + FromBytes<Bytes = [u8; N]>,
{
    type Error = UnpackError;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let bytes = Unpack::unpack(buf)?;
        let num = FromBytes::from_le_bytes(&bytes);
        Ok(LittleEndian(num))
    }
}

impl<const N: usize, T> Pack for BigEndian<T>
where
    T: ToBytes<Bytes = [u8; N]> + FromBytes,
{
    fn pack_into(&self, buf: &mut Vec<u8>) {
        self.to_be_bytes().pack_into(buf);
    }
}

impl<const N: usize, T> Pack for LittleEndian<T>
where
    T: ToBytes<Bytes = [u8; N]> + FromBytes,
{
    fn pack_into(&self, buf: &mut Vec<u8>) {
        self.to_le_bytes().pack_into(buf);
    }
}
