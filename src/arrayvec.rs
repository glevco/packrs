use crate::{Unpack, UnpackError, UnpackLength};
use arrayvec::{ArrayString, ArrayVec};

impl<'a, const CAP: usize> UnpackLength<'a> for ArrayString<CAP> {
    type Error = UnpackError;

    #[inline]
    fn unpack(buf: &mut &[u8], len: usize) -> Result<Self, Self::Error> {
        if len > CAP {
            return Err(UnpackError::CapacityError {
                capacity: CAP,
                found: len,
            });
        }

        let str = UnpackLength::unpack(buf, len)?;
        Ok(ArrayString::from(str).unwrap())
    }
}

impl<'a, const CAP: usize, U> UnpackLength<'a> for ArrayVec<U, CAP>
where
    U: Unpack<'a>,
    U::Error: From<UnpackError>,
{
    type Error = U::Error;

    #[inline]
    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        if len > CAP {
            Err(UnpackError::CapacityError {
                capacity: CAP,
                found: len,
            })?
        }

        let mut items = ArrayVec::new();
        for _ in 0..len {
            items.push(U::unpack(buf)?)
        }

        Ok(items)
    }
}

impl<const CAP: usize> Unpack<'_> for ArrayString<CAP> {
    type Error = UnpackError;

    #[inline]
    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let array: [u8; CAP] = Unpack::unpack(buf)?;
        Ok(ArrayString::from_byte_string(&array)?)
    }
}

impl<'a, const CAP: usize, U: Unpack<'a>> Unpack<'a> for ArrayVec<U, CAP> {
    type Error = U::Error;

    #[inline]
    fn unpack(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
        let array: [U; CAP] = Unpack::unpack(buf)?;
        Ok(ArrayVec::from(array))
    }
}
