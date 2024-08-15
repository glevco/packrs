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

// TODO
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_be() -> Result<(), UnpackError> {
        let data: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

        let buf = &mut &data[..];
        let result: BigEndian<u8> = Unpack::unpack(buf)?;
        assert_eq!(*result, 0x12);
        assert_eq!(buf, &[0x34, 0x56, 0x78]);

        let buf = &mut &data[..];
        let result: BigEndian<u16> = Unpack::unpack(buf)?;
        assert_eq!(*result, 0x1234);
        assert_eq!(buf, &[0x56, 0x78]);

        let buf = &mut &data[..];
        let result: BigEndian<u32> = Unpack::unpack(buf)?;
        assert_eq!(*result, 0x12345678);
        assert_eq!(buf, &[]);

        Ok(())
    }

    #[test]
    fn it_fails_be() {
        let data: [u8; 1] = [0x12];
        let buf = &mut &data[..];
        let result: Result<BigEndian<u16>, UnpackError> = Unpack::unpack(buf);

        assert_eq!(
            result.unwrap_err(),
            UnpackError::NotEnoughBytes {
                expected: 2,
                found: 1
            }
        );
    }

    #[test]
    fn it_works_le() -> Result<(), UnpackError> {
        let data: [u8; 3] = [0x12, 0x34, 0x56];
        let buf = &mut &data[..];
        let result: LittleEndian<u16> = Unpack::unpack(buf)?;

        assert_eq!(*result, 0x3412);
        assert_eq!(buf, &[0x56]);
        Ok(())
    }

    #[test]
    fn it_fails_le() {
        let data: [u8; 1] = [0x12];
        let buf = &mut &data[..];
        let result: Result<LittleEndian<u16>, UnpackError> = Unpack::unpack(buf);

        assert_eq!(
            result.unwrap_err(),
            UnpackError::NotEnoughBytes {
                expected: 2,
                found: 1
            }
        );
    }
}
