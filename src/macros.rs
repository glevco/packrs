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
            let buf = $crate::pack!($first);
            $crate::pack_into!(buf, $($rest)+);
            buf
        }
    };
}

#[macro_export]
macro_rules! unpack {
    ($buf:expr, {}) => {};

    ($buf:expr, { let $name:ident: $T:ty; $($tail:tt)* }) => {
        let $name: $T = $crate::Unpack::unpack($buf)?;
        $crate::unpack!($buf, { $($tail)* })
    };

    ($buf:expr, { let $name:ident: $T:ty as $Prim:ty; $($tail:tt)* }) => {
        let $name: $T = $crate::Unpack::unpack($buf)?;
        let $name = *$name as $Prim;
        $crate::unpack!($buf, { $($tail)* })
    };

    ($buf:expr, { let $name:ident: $T:ty, $len:expr; $($tail:tt)* }) => {
        let $name: $T = $crate::UnpackLength::unpack($buf, $len)?;
        $crate::unpack!($buf, { $($tail)* })
    };
}
