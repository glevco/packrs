use paste::paste;

#[macro_export]
macro_rules! pack_into {
    ($buf:expr, $($value:expr),+ $(,)?) => {
        $($crate::Pack::pack_into(&$value, $buf);)+
    };
}

#[macro_export]
macro_rules! pack {
    ($single:expr $(,)?) => {
        $crate::Pack::pack(&$single)
    };

    ($first:expr, $($rest:expr),+ $(,)?) => {
        {
            let mut buf = $crate::pack!($first);
            $($crate::pack_into!(&mut buf, $rest);)+
            buf
        }
    };
}

#[macro_export]
macro_rules! impl_unpack_n {
    ($($n:literal,)? $($ty:ident),+) => {
        paste! {
            pub fn [<unpack $($n)?>]<'a, $($ty,)+ $([<E $ty>],)+ E>(buf: &mut &'a[u8]) -> Result<($($ty),+), E>
            where
                $(E: From<[<E $ty>]>,)+
                $($ty: $crate::Unpack<'a, Error = [<E $ty>]>,)+
            {
                Ok(($($ty::unpack(buf)?),+))
            }
        }
    };
}

impl_unpack_n!(2, T1, T2);
impl_unpack_n!(3, T1, T2, T3);
impl_unpack_n!(4, T1, T2, T3, T4);
impl_unpack_n!(5, T1, T2, T3, T4, T5);
impl_unpack_n!(6, T1, T2, T3, T4, T5, T6);
impl_unpack_n!(7, T1, T2, T3, T4, T5, T6, T7);
impl_unpack_n!(8, T1, T2, T3, T4, T5, T6, T7, T8);

#[macro_export]
macro_rules! unpack {
    ($buf:expr, ($($ty:ty),+)) => {
        ($(<$ty as $crate::Unpack>::unpack($buf)?),+)
    };
}

// TODO: legacy
// #[macro_export]
// macro_rules! unpack {
//     ($buf:expr, {}) => {};
//
//     ($buf:expr, { let $name:ident: $T:ty; $($tail:tt)* }) => {
//         let $name: $T = $crate::Unpack::unpack($buf)?;
//         $crate::unpack!($buf, { $($tail)* })
//     };
//
//     ($buf:expr, { let $name:ident: $T:ty as $Prim:ty; $($tail:tt)* }) => {
//         let $name: $T = $crate::Unpack::unpack($buf)?;
//         let $name = *$name as $Prim;
//         $crate::unpack!($buf, { $($tail)* })
//     };
//
//     ($buf:expr, { let $name:ident: $T:ty, $len:expr; $($tail:tt)* }) => {
//         let $name: $T = $crate::UnpackLength::unpack($buf, $len)?;
//         $crate::unpack!($buf, { $($tail)* })
//     };
// }
