use arrayvec::{ArrayString, ArrayVec};
use num_traits::FromBytes;
use std::fmt::Debug;
use std::ops::Deref;
use thiserror::Error;

// TODO: ArrayVec and Hex

#[derive(Error, Debug, PartialEq)]
pub(crate) enum UnpackError {
    #[error("Too many bytes to unpack. Expected {expected} bytes, found {found}.")]
    TooManyBytes { expected: usize, found: usize },
    #[error("Error decoding hex bytes.")]
    DecodeHex(#[from] hex::FromHexError),
}

impl<'a, const N: usize> UnpackLength<'a> for ArrayVec<u8, N> {
    type Error = UnpackError;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        if len > N {
            Err(UnpackError::TooManyBytes {
                expected: N,
                found: len,
            })
        } else {
            let bytes: &[u8] = UnpackLength::unpack(buf, len)?;
            Ok(ArrayVec::try_from(bytes).unwrap())
        }
    }
}

impl<'a, const N: usize> UnpackLength<'a> for ArrayString<N> {
    type Error = UnpackError;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        if len > N {
            Err(UnpackError::TooManyBytes {
                expected: N,
                found: len,
            })
        } else {
            let str = UnpackLength::unpack(buf, len)?;
            Ok(ArrayString::from(str).unwrap())
        }
    }
}

impl<'a, const N: usize, T, E> UnpackLength<'a> for ArrayVec<T, N>
where
    E: From<UnpackError>,
    T: Unpack<Error = E>,
{
    type Error = E;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        if len > N {
            Err(UnpackError::TooManyBytes {
                expected: N,
                found: len,
            }
            .into())
        } else {
            let items = <Vec<T>>::unpack(buf, len)?;
            Ok(ArrayVec::from_iter(items))
        }
    }
}


impl<const N: usize> Unpack for ArrayVec<u8, N> {
    type Error = UnpackError;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let array: [u8; N] = Unpack::unpack(buf)?;
        Ok(ArrayVec::from(array))
    }
}

impl<const N: usize> Unpack for ArrayString<N> {
    type Error = UnpackError;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let array = <[u8; N]>::unpack(buf)?;
        Ok(ArrayString::from_byte_string(&array)?)
    }
}

impl<const N: usize, T: Unpack> Unpack for ArrayVec<T, N> {
    type Error = T::Error;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let array = <[T; N]>::unpack(buf)?;
        Ok(ArrayVec::from(array))
    }
}

pub(crate) trait FromHex: Sized {
    type Error;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error>;
}

impl<E, U> FromHex for U
where
    E: From<UnpackError>,
    U: Unpack<Error = E>,
{
    type Error = E;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let bytes = hex::decode(hex).map_err(UnpackError::from)?;
        U::unpack(&mut bytes.as_slice())
    }
}

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

    #[test]
    fn test_unpack_slice() -> Result<(), UnpackError> {
        let data = [0x12, 0x34, 0x56, 0x78];
        let buf = &mut &data[..];
        let result: &[u8] = UnpackLength::unpack(buf, 3)?;

        assert_eq!(result, &[0x12, 0x34, 0x56]);
        assert_eq!(buf, &[0x78]);

        Ok(())
    }

    #[test]
    fn test_unpack_byte_array() -> Result<(), UnpackError> {
        let data = [0x12, 0x34, 0x56, 0x78];
        let buf = &mut &data[..];
        let result: [u8; 3] = Unpack::unpack(buf)?;

        assert_eq!(result, [0x12, 0x34, 0x56]);
        assert_eq!(buf, &[0x78]);

        Ok(())
    }
}
