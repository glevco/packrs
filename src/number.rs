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
//  - property testing with quickcheck/arbitrary
//  - test failures
#[cfg(test)]
mod tests {
    use paste::paste;
    use super::*;

    const DATA: [u8; 16] = [
        0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
        0xFF, 0xED, 0xCB, 0xA9, 0x87, 0x65, 0x43, 0x21,
    ];

    macro_rules! impl_test_unpack_number {
        ($num:ty, $end:ident, $expected:literal) => {
            paste!{
                #[test]
                fn [<test_unpack_ $num _ $end:lower>]() -> Result<(), UnpackError> {
                    let buf = &mut &DATA[..];
                    let result: $end<$num> = Unpack::unpack(buf)?;

                    assert_eq!(*result, $expected as $num);
                    Ok(())
                }
            }
        };

        ($num:ty, be = $be:literal, le = $le:literal) => {
            impl_test_unpack_number!($num, BigEndian, $be);
            impl_test_unpack_number!($num, LittleEndian, $le);
        };
    }

    impl_test_unpack_number!(u8, be = 0x12, le = 0x12);
    impl_test_unpack_number!(u16, be = 0x1234, le = 0x3412);
    impl_test_unpack_number!(u32, be = 0x12345678, le = 0x78563412);
    impl_test_unpack_number!(u64, be = 0x123456789ABCDEF0, le = 0xF0DEBC9A78563412);
    impl_test_unpack_number!(u128, be = 0x123456789abcdef0ffedcba987654321, le = 0x21436587a9cbedfff0debc9a78563412);

    impl_test_unpack_number!(i8, be = 0x12, le = 0x12);
    impl_test_unpack_number!(i16, be = 0x1234, le = 0x3412);
    impl_test_unpack_number!(i32, be = 0x12345678, le = 0x78563412);
    impl_test_unpack_number!(i64, be = 0x123456789ABCDEF0, le = -0xf21436587a9cbee);
    impl_test_unpack_number!(i128, be = 0x123456789abcdef0ffedcba987654321, le = 0x21436587a9cbedfff0debc9a78563412);

    impl_test_unpack_number!(f32, be = 5.690456613903524e-28, le = 1.7378244361449504e34);
    impl_test_unpack_number!(f64, be = 5.626349274901198e-221, le = -4.886459655043775e+235);











    // #[test]
    // fn it_works_be() -> Result<(), UnpackError> {
    //     let data: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
    //
    //     let buf = &mut &data[..];
    //     let result: BigEndian<u8> = Unpack::unpack(buf)?;
    //     assert_eq!(*result, 0x12);
    //     assert_eq!(buf, &[0x34, 0x56, 0x78]);
    //
    //     let buf = &mut &data[..];
    //     let result: BigEndian<u16> = Unpack::unpack(buf)?;
    //     assert_eq!(*result, 0x1234);
    //     assert_eq!(buf, &[0x56, 0x78]);
    //
    //     let buf = &mut &data[..];
    //     let result: BigEndian<u32> = Unpack::unpack(buf)?;
    //     assert_eq!(*result, 0x12345678);
    //     assert_eq!(buf, &[]);
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn it_fails_be() {
    //     let data: [u8; 1] = [0x12];
    //     let buf = &mut &data[..];
    //     let result: Result<BigEndian<u16>, UnpackError> = Unpack::unpack(buf);
    //
    //     assert_eq!(
    //         result.unwrap_err(),
    //         UnpackError::NotEnoughBytes {
    //             expected: 2,
    //             found: 1
    //         }
    //     );
    // }
    //
    // #[test]
    // fn it_works_le() -> Result<(), UnpackError> {
    //     let data: [u8; 3] = [0x12, 0x34, 0x56];
    //     let buf = &mut &data[..];
    //     let result: LittleEndian<u16> = Unpack::unpack(buf)?;
    //
    //     assert_eq!(*result, 0x3412);
    //     assert_eq!(buf, &[0x56]);
    //     Ok(())
    // }
    //
    // #[test]
    // fn it_fails_le() {
    //     let data: [u8; 1] = [0x12];
    //     let buf = &mut &data[..];
    //     let result: Result<LittleEndian<u16>, UnpackError> = Unpack::unpack(buf);
    //
    //     assert_eq!(
    //         result.unwrap_err(),
    //         UnpackError::NotEnoughBytes {
    //             expected: 2,
    //             found: 1
    //         }
    //     );
    // }
}
