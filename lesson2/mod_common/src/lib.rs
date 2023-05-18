#![no_std]

pub trait Module {
    type Entry;
}

pub type Identity<T> = T;

#[cfg(not(feature = "passive"))]
#[macro_export]
macro_rules! entry {
    ($init_fn:expr, $type:ty) => {
        #[link_section = ".init_calls"]
        #[used]
        static ENTRY: $type = mod_common::Identity::<$type> { init_fn: $init_fn };
    };
}

#[cfg(feature = "passive")]
#[macro_export]
macro_rules! entry {
    ($init_fn:expr, $type:ty) => {
        pub const ENTRY: $type = mod_common::Identity::<$type> { init_fn: $init_fn };
    };
}

pub const fn one<T>(_: &T) -> usize {
    return 1;
}

#[cfg(feature = "passive")]
#[macro_export]
macro_rules! entries {
    ($var:ident, $type:ty $(, $m:ident)*) => {
        static $var : [<$type as mod_common::Module>::Entry; 0 $( + one(& $m :: ENTRY) )* ] = [ $( $m :: ENTRY ),* ];
    };
}

#[macro_export]
macro_rules! module {
    (type: $type:ty $(, $name:ident: $value:expr) * ) => {
        type Mod = $type;
        type Entry = <$type as mod_common::Module>::Entry;
        mod_common::entry! (init_fn, Entry);
        fn init_fn() -> Mod {
            $( let $name = $value; )*
            Mod { $($name,)* }
        }
    };
}
