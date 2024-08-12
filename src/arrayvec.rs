use arrayvec::{ArrayString, ArrayVec};
use crate::{Unpack, UnpackError, UnpackLength};

impl<'a, const CAP: usize> UnpackLength<'a> for ArrayVec<u8, CAP> {
    type Error = UnpackError;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        if len > CAP {
            return Err(UnpackError::CapacityError {
                capacity: CAP,
                found: len,
            })
        }

        let bytes: &[u8] = UnpackLength::unpack(buf, len)?;
        Ok(ArrayVec::try_from(bytes).unwrap())
    }
}

impl<'a, const CAP: usize> UnpackLength<'a> for ArrayString<CAP> {
    type Error = UnpackError;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        if len > CAP {
            return Err(UnpackError::CapacityError {
                capacity: CAP,
                found: len,
            })
        }

        let str = UnpackLength::unpack(buf, len)?;
        Ok(ArrayString::from(str).unwrap())
    }
}

impl<'a, const CAP: usize, U, E> UnpackLength<'a> for ArrayVec<U, CAP>
where
    E: From<UnpackError>,
    U: Unpack<'a, Error = E>,
{
    type Error = E;

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


impl<const CAP: usize> Unpack<'_> for ArrayVec<u8, CAP> {
    type Error = UnpackError;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let array: [u8; CAP] = Unpack::unpack(buf)?;
        Ok(ArrayVec::from(array))
    }
}

impl<const CAP: usize> Unpack<'_> for ArrayString<CAP> {
    type Error = UnpackError;

    fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
        let array: [u8; CAP] = Unpack::unpack(buf)?;
        Ok(ArrayString::from_byte_string(&array)?)
    }
}

impl<'a, const CAP: usize, U: Unpack<'a>> Unpack<'a> for ArrayVec<U, CAP> {
    type Error = U::Error;

    fn unpack(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
        let array: [U; CAP] = Unpack::unpack(buf)?;
        Ok(ArrayVec::from(array))
    }
}