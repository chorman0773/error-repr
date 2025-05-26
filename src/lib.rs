#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![deny(missing_docs)]
//! A crate that generlizes the API surface of `std::io::Error` by providing a type that wraps an error kind and a payload.
//!
//! ## Features
//! * `alloc`: Adds support for interfaces that require an allocator. In particular, this adds error constructors that take arbitrary payloads.
//! * `std`: Adds support for interfaces that require the standard library (conversions to and from `std::io::Error` and an implementing [`kind::ErrorKind`] for `std::io::ErrorKind`).
//! * `error-track_caller`: Causes construction of [`Error`] to carry information about the location they were created.
//!     This is a debugging tool only - it does not add any API interfaces to the crate. Instead, the error simply holds the information internally, which is shown in the [`core::fmt::Debug`] impl.

mod error;
/// Module for the [`ErrorKind`][kind::ErrorKind] trait and support traits.
pub mod kind;

pub use error::Error;

#[cfg(feature = "alloc")]
extern crate alloc;

cfg_match::cfg_match! {
    any(target_os = "windows", target_os = "lilium") => {
        /// The raw type of an OS Error.
        /// On most targets, this is `i32`, but on other targets it may be `isize` or another integer type
        pub type RawOsError = isize;
    }
    _ => {
        /// The raw type of an OS Error.
        /// On most targets, this is `i32`, but on other targets it may be `isize` or another integer type
        pub type RawOsError = i32;
    }
}
