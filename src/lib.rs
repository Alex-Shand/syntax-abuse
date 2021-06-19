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
pub use pretty_assertions::{assert_eq, assert_ne};

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
    ($i:ident = $e:expr => $($context:tt)+) => {
        match $e {
            $i => $($context)+
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
            use $crate::{ tests, testcase, testdata, assert_eq, assert_ne };
            use super::*;
            $($tests)*
        }
    };
    ($($tests:tt)*) => { ::syntax_abuse::tests!{tests: $($tests)* } }
}

/// Lazy static wrapper for test data
#[macro_export]
macro_rules! testdata {
    ($($name:ident : $ty:ty = $expr:expr;)+) => {
        $crate::lazy_static! {
            $(static ref $name: $ty = $expr;)+
        }
    }
}

/// Test method boilerplate
#[macro_export]
macro_rules! testcase {
    ($name:ident, $test:expr, $expected:expr) => {
        #[test]
        fn $name() {
            match ($test, $expected) {
                (test, expected) => assert_eq!(
                    test,
                    expected,
                    "{} != {}",
                    stringify!($test),
                    stringify!($expected)
                ),
            }
        }
    };
}
