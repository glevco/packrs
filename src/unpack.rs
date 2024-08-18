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
    #[cfg(feature = "arrayvec")]
    #[error("too many bytes to unpack (capacity {capacity}, found {found})")]
    CapacityError { capacity: usize, found: usize },
    #[cfg(feature = "hex")]
    #[error("error decoding hex bytes")]
    FromHexError(#[from] hex::FromHexError),
}

fn split_buf(buf: &[u8], len: usize) -> Result<(&[u8], &[u8]), UnpackError> {
    buf.split_at_checked(len)
        .ok_or(UnpackError::NotEnoughBytes {
            expected: len,
            found: buf.len(),
        })
}

impl<'a> UnpackLength<'a> for &'a [u8] {
    type Error = UnpackError;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        let (len_bytes, rest) = split_buf(buf, len)?;
        *buf = rest;
        Ok(len_bytes)
    }
}

impl<'a> UnpackLength<'a> for &'a str {
    type Error = UnpackError;

    fn unpack(buf: &mut &'a [u8], len: usize) -> Result<Self, Self::Error> {
        let (len_bytes, rest) = split_buf(buf, len)?;
        let str = std::str::from_utf8(len_bytes)?;
        *buf = rest;
        Ok(str)
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
        let original = *buf;
        let mut items = Vec::with_capacity(len);
        for _ in 0..len {
            match U::unpack(buf) {
                Ok(item) => items.push(item),
                Err(err) => {
                    *buf = original;
                    return Err(err);
                }
            }
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

impl<'a, const N: usize, U: Unpack<'a>> Unpack<'a> for [U; N] {
    type Error = U::Error;

    fn unpack(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
        let items: Vec<U> = UnpackLength::unpack(buf, N)?;
        Ok(items.try_into().ok().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Test peek

    #[test]
    fn test_unpack_byte_slice_not_enough_bytes() {
        let data = [1, 2, 3, 4];
        let buf = &mut &data[..];
        let result: Result<&[u8], UnpackError> = UnpackLength::unpack(buf, 5);

        assert_eq!(
            result.unwrap_err(),
            UnpackError::NotEnoughBytes {
                expected: 5,
                found: 4,
            }
        );
        assert_eq!(buf, &data);
    }

    #[test]
    fn test_unpack_byte_slice_success() -> Result<(), UnpackError> {
        let data = [1, 2, 3, 4];
        let buf = &mut &data[..];
        let result: &[u8] = UnpackLength::unpack(buf, 3)?;

        assert_eq!(result, &[1, 2, 3]);
        assert_eq!(buf, &[4]);
        Ok(())
    }

    #[test]
    fn test_unpack_str_not_enough_bytes() {
        let data = b"abcd";
        let buf = &mut &data[..];
        let result: Result<&str, UnpackError> = UnpackLength::unpack(buf, 5);

        assert_eq!(
            result.unwrap_err(),
            UnpackError::NotEnoughBytes {
                expected: 5,
                found: 4,
            }
        );
        assert_eq!(buf, data);
    }

    #[test]
    fn test_unpack_str_utf8_error() {
        let data = [0, 159, 146, 150];
        let buf = &mut &data[..];
        let result: Result<&str, UnpackError> = UnpackLength::unpack(buf, 3);

        assert!(matches!(result.unwrap_err(), UnpackError::Utf8Error(_)));
        assert_eq!(buf, &data);
    }

    #[test]
    fn test_unpack_str_success() -> Result<(), UnpackError> {
        let data = b"abcd";
        let buf = &mut &data[..];
        let result: &str = UnpackLength::unpack(buf, 3)?;

        assert_eq!(result, "abc");
        assert_eq!(buf, b"d");
        Ok(())
    }

    #[test]
    fn test_unpack_byte_vec_not_enough_bytes() {
        let data = [1, 2, 3, 4];
        let buf = &mut &data[..];
        let result: Result<Vec<u8>, UnpackError> = UnpackLength::unpack(buf, 5);

        assert_eq!(
            result.unwrap_err(),
            UnpackError::NotEnoughBytes {
                expected: 5,
                found: 4,
            }
        );
        assert_eq!(buf, &data);
    }

    #[test]
    fn test_unpack_byte_vec_success() -> Result<(), UnpackError> {
        let data = [1, 2, 3, 4];
        let buf = &mut &data[..];
        let result: Vec<u8> = UnpackLength::unpack(buf, 3)?;

        assert_eq!(result, vec![1, 2, 3]);
        assert_eq!(buf, &[4]);
        Ok(())
    }

    #[test]
    fn test_unpack_string_not_enough_bytes() {
        let data = b"abcd";
        let buf = &mut &data[..];
        let result: Result<String, UnpackError> = UnpackLength::unpack(buf, 5);

        assert_eq!(
            result.unwrap_err(),
            UnpackError::NotEnoughBytes {
                expected: 5,
                found: 4,
            }
        );
        assert_eq!(buf, data);
    }

    #[test]
    fn test_unpack_string_utf8_error() {
        let data = [0, 159, 146, 150];
        let buf = &mut &data[..];
        let result: Result<String, UnpackError> = UnpackLength::unpack(buf, 3);

        assert!(matches!(result.unwrap_err(), UnpackError::Utf8Error(_)));
        assert_eq!(buf, &data);
    }

    #[test]
    fn test_unpack_string_success() -> Result<(), UnpackError> {
        let data = b"abcd";
        let buf = &mut &data[..];
        let result: String = UnpackLength::unpack(buf, 3)?;

        assert_eq!(result, String::from("abc"));
        assert_eq!(buf, b"d");
        Ok(())
    }

    #[test]
    fn test_unpack_byte_array_not_enough_bytes() {
        let data = [1, 2, 3, 4];
        let buf = &mut &data[..];
        let result: Result<[u8; 5], UnpackError> = Unpack::unpack(buf);

        assert_eq!(
            result.unwrap_err(),
            UnpackError::NotEnoughBytes {
                expected: 5,
                found: 4,
            }
        );
        assert_eq!(buf, &data);
    }

    #[test]
    fn test_unpack_byte_array_success() -> Result<(), UnpackError> {
        let data = [1, 2, 3, 4];
        let buf = &mut &data[..];
        let result: [u8; 3] = Unpack::unpack(buf)?;

        assert_eq!(result, [1, 2, 3]);
        assert_eq!(buf, &[4]);
        Ok(())
    }

    #[derive(Debug, PartialEq)]
    struct TestStruct([u8; 1]);

    #[derive(Debug, PartialEq)]
    struct TestError(UnpackError);

    impl From<UnpackError> for TestError {
        fn from(error: UnpackError) -> Self {
            Self(error)
        }
    }

    impl Unpack<'_> for TestStruct {
        type Error = TestError;

        fn unpack(buf: &mut &[u8]) -> Result<Self, Self::Error> {
            Ok(Self(Unpack::unpack(buf)?))
        }
    }

    #[test]
    fn test_unpack_custom_not_enough_bytes() {
        let data = [];
        let buf = &mut &data[..];
        let result: Result<TestStruct, TestError> = Unpack::unpack(buf);

        assert_eq!(
            result.unwrap_err(),
            TestError(UnpackError::NotEnoughBytes {
                expected: 1,
                found: 0,
            })
        );
        assert_eq!(buf, &data);
    }

    #[test]
    fn test_unpack_custom_success() -> Result<(), TestError> {
        let data = [1, 2, 3, 4];
        let buf = &mut &data[..];
        let result: TestStruct = Unpack::unpack(buf)?;

        assert_eq!(result, TestStruct([1]));
        assert_eq!(buf, &[2, 3, 4]);
        Ok(())
    }

    #[test]
    fn test_unpack_custom_vec_not_enough_bytes() {
        let data = [1, 2, 3, 4];
        let buf = &mut &data[..];
        let result: Result<Vec<TestStruct>, TestError> = UnpackLength::unpack(buf, 5);

        assert_eq!(
            result.unwrap_err(),
            TestError(UnpackError::NotEnoughBytes {
                expected: 1,
                found: 0,
            })
        );
        assert_eq!(buf, &data);
    }

    #[test]
    fn test_unpack_custom_vec_success() -> Result<(), TestError> {
        let data = [1, 2, 3, 4];
        let buf = &mut &data[..];
        let result: Vec<TestStruct> = UnpackLength::unpack(buf, 3)?;

        assert_eq!(
            result,
            vec![TestStruct([1]), TestStruct([2]), TestStruct([3])]
        );
        assert_eq!(buf, &[4]);
        Ok(())
    }

    #[test]
    fn test_unpack_custom_array_not_enough_bytes() {
        let data = [1, 2, 3, 4];
        let buf = &mut &data[..];
        let result: Result<[TestStruct; 5], TestError> = Unpack::unpack(buf);

        assert_eq!(
            result.unwrap_err(),
            TestError(UnpackError::NotEnoughBytes {
                expected: 1,
                found: 0,
            })
        );
        assert_eq!(buf, &data);
    }

    #[test]
    fn test_unpack_custom_array_success() -> Result<(), TestError> {
        let data = [1, 2, 3, 4];
        let buf = &mut &data[..];
        let result: [TestStruct; 3] = Unpack::unpack(buf)?;

        assert_eq!(result, [TestStruct([1]), TestStruct([2]), TestStruct([3])]);
        assert_eq!(buf, &[4]);
        Ok(())
    }
}
