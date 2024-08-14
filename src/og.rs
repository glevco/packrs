use num_traits::FromBytes;
use std::fmt::Debug;
use std::ops::Deref;
use thiserror::Error;

// TODO: tests

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
