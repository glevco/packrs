use crate::{Unpack, UnpackError};

pub trait UnpackHex: for<'a> Unpack<'a, Error = <Self as UnpackHex>::Error> {
    type Error: From<UnpackError>;

    fn from_hex(hex: impl AsRef<[u8]>) -> Result<Self, <Self as UnpackHex>::Error> {
        let bytes = hex::decode(hex).map_err(UnpackError::from)?;
        Self::unpack(&mut &bytes[..])
    }
}

impl<U, E> UnpackHex for U
where
    U: for<'a> Unpack<'a, Error = E>,
    E: From<UnpackError>,
{
    type Error = E;
}
