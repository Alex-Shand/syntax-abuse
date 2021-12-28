//! Misc `macro_rules` macros
#![deny(elided_lifetimes_in_paths)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(noop_method_call)]
#![deny(unreachable_pub)]
#![deny(unused_crate_dependencies)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![deny(unused_qualifications)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_results)]
#![deny(clippy::pedantic)]

pub use lazy_static::lazy_static;
pub use pretty_assertions;
pub use proptest;

/// Generate a `From` or `TryFrom` impl
#[macro_export]
macro_rules! conversion {
    (try<$error:ty> $src:ty => $dst:ty = $body:expr) => {
        $crate::conversion!{try<$error> $src => $dst [] = $body}
    };
    (try<$error:ty> $src:ty => $dst:ty [$($generics:tt)*] = $body:expr) => {
        impl<$($generics)*> ::std::convert::TryFrom<$src> for $dst {
            type Error = $error;
            fn try_from(value: $src) -> Result<Self, Self::Error> {
                $body(value)
            }
        }
    };
    ($src:ty => $dst:ty = $body:expr) => {
        $crate::conversion!{$src => $dst [] = $body}
    };
    ($src:ty => $dst:ty [$($generics:tt)*] = $body:expr) => {
        impl<$($generics)*> From<$src> for $dst {
            fn from(value: $src) -> Self {
                $body(value)
            }
        }
    }
}

/// Force evaluation of an expr fragment
#[macro_export]
macro_rules! eval {
    ($pat:pat = $e:expr => $($context:tt)+) => {
        match $e {
            $pat => $($context)+
        }
    }
}

/// Compute the length of a variadic parameter
#[macro_export]
macro_rules! length_of {
    () => { 0 };
    ($head:ident $($tail:ident)*) => { $crate::length_of!($($tail)*) + 1 }
}

/// Generate a trivial getter
#[macro_export]
macro_rules! get {
    ($vis:vis $name:ident : $type:ty) => {
        $vis fn $name(&self) -> &$type {
            &self.$name
        }
    }
}

/// Test module boilerplate
#[macro_export]
macro_rules! tests {
    ($name:ident : $($tests:tt)*) => {
        #[cfg(test)]
        mod $name {
            use $crate::{ tests, testcase, testdata };
            use super::*;
            $($tests)*
        }
    };
    ($($tests:tt)*) => { ::syntax_abuse::tests!{tests: $($tests)* } }
}

/// do while loop
#[macro_export]
macro_rules! do_while {
    (do $body: block while $e: expr) => {
        $body
        while $e $body
    }
}

/// Lazy static wrapper for test data
#[macro_export]
macro_rules! testdata {
    // For types that can't be named
    ($($name:ident : ??? = $expr:expr;)+) => {
        $(
            macro_rules! $name {
                () => { $expr }
            }
        )+
    };
    ($($name:ident : $ty:ty = $expr:expr;)+) => {
        $crate::lazy_static! {
            $(static ref $name: $ty = $expr;)+
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! body {
    ([$($eq:tt)+],
     [
         $(let $pat:pat = $expr:expr;)*
             ($test:expr) should be $(equal to)? ($expected:expr) $(;)?
     ]
    ) => {
        $(let $pat = $expr;)*
        $crate::eval! {
            (test, expected) = ($test, $expected) =>
                $($eq)+!(test, expected)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! should_panic {
    () => { #[should_panic] };
    ($msg:literal) => { #[should_panic(expected = $msg)] };
}

#[doc(hidden)]
#[macro_export]
macro_rules! drop {
    ($test:expr, $ignored:expr) => {
        ::std::mem::drop($test)
    }
}

/// Test method boilerplate
#[macro_export]
macro_rules! testcase {
    ($name:ident($($arg:ident in $range:path),* $(,)?) $(require ($require:expr))? {
        $($tt:tt)+
    }) => {
        #[test]
        fn $name() {
            use $crate::proptest;
            $crate::proptest::proptest!(|($($arg in $range),*)| {
                $($crate::proptest::prop_assume!($require);)?
                $crate::body!([$crate::proptest::prop_assert_eq], [$($tt)+])
            });
        }
    };
    ($name:ident panics { $($tt:tt)+ }) => {
        #[test]
        #[should_panic]
        #[allow(clippy::drop_copy)]
        fn $name() {
            $crate::body!([$crate::drop], [($($tt)+) should be (true)])
        }
    };
    ($name:ident panics with $msg:literal{ $($tt:tt)+ }) => {
        #[test]
        #[should_panic(expected = $msg)]
        #[allow(clippy::drop_copy)]
        fn $name() {
            $crate::body!([$crate::drop], [($($tt)+) should be (true)])
        }
    };
    ($name:ident { $($tt:tt)+ }) => {
        #[test]
        fn $name() {
            $crate::body!([$crate::pretty_assertions::assert_eq], [$($tt)+]);
        }
    };
}
