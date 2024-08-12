use crate::pack::Pack;
use crate::unpack::{Unpack, UnpackError};
use num_traits::{FromBytes, ToBytes};
use std::ops::Deref;

macro_rules! impl_number {
    ($name:ident, $from_bytes:ident, $to_bytes:ident) => {
        #[derive(Debug)]
        pub struct $name<T: FromBytes + ToBytes>(T);

        impl<T: FromBytes + ToBytes> Deref for $name<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> From<T> for $name<T>
        where
            T: FromBytes + ToBytes,
        {
            fn from(value: T) -> Self {
                Self(value)
            }
        }

        impl<const N: usize, T> Unpack<'_> for $name<T>
        where
            T: ToBytes + FromBytes<Bytes = [u8; N]>,
        {
            type Error = UnpackError;

            fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
                let bytes = Unpack::unpack(buf)?;
                let num = FromBytes::$from_bytes(&bytes);
                Ok(Self(num))
            }
        }

        impl<const N: usize, T> Pack for $name<T>
        where
            T: ToBytes<Bytes = [u8; N]> + FromBytes,
        {
            fn pack_into(&self, buf: &mut Vec<u8>) {
                self.$to_bytes().pack_into(buf);
            }
        }
    };
}

impl_number!(BigEndian, from_be_bytes, to_be_bytes);
impl_number!(LittleEndian, from_le_bytes, to_le_bytes);
