#[macro_export]
macro_rules! unpack {
    ($buf:expr, {}) => {};

    ($buf:expr, { let $name:ident: $T:ty; $($tail:tt)* }) => {
        let $name: $T = Unpack::unpack($buf)?;
        $crate::unpack!($buf, { $($tail)* })
    };

    ($buf:expr, { let $name:ident: $T:ty as $Prim:ty; $($tail:tt)* }) => {
        let $name: $T = Unpack::unpack($buf)?;
        let $name = *$name as $Prim;
        $crate::unpack!($buf, { $($tail)* })
    };

    ($buf:expr, { let $name:ident: $T:ty, $len:expr; $($tail:tt)* }) => {
        let $name: $T = UnpackLength::unpack($buf, $len)?;
        $crate::unpack!($buf, { $($tail)* })
    };
}
