use super::RawOsError;

/// Base trait for the kind of [`Error`][crate::Error]s.
///
/// Error Kinds are simple types (usually flat enums) that describes the overall category of an error (with the payload being encoded in the [`Error`][crate::Error] type instead).
/// Because [`ErrorKind`] implementors are simple, they are expected to implement [`Copy`], as well as [`Debug`][core::fmt::Debug] and [`Display`][core::fmt::Display].
pub trait ErrorKind: core::fmt::Debug + core::fmt::Display + Copy {
    /// A Constant for the [`ErrorKind`] that is an "Other" value
    const OTHER: Self;

    /// Returns a value of the [`ErrorKind`] that represents an uncategorized Error.
    /// This should normally not be matachable outside of the crate that defines the [`ErrorKind`]
    fn uncategorized() -> Self;
}

/// Trait for [`ErrorKind`]s that can be created from a [`RawOsError`]
pub trait FromRawOsError: ErrorKind {
    /// Returns an Error value that corresponds to the OS error code.
    ///
    /// ## Warning
    /// Implementation of this function is target specific. OS Errors are inherently per-OS and this must be taken into account when implemented.
    fn from_raw_os_error(raw: RawOsError) -> Self;
}

/// Trait for [`ErrorKind`]s that can be converted from [`std::io::ErrorKind`]. Implementing this trait allows converting from [`std::io::Error`] to [`Error<Self>`][crate::Error]
#[cfg(feature = "std")]
pub trait FromIoKind: ErrorKind {
    /// Produces a value of the [`ErrorKind`] that corresponds to `kind`
    ///
    /// Except when mapping to an uncategorized error, this method is expected to be consistent with [`IntoIoKind::into_io_error_kind`] and [`FromRawOsError::from_raw_os_error`].
    /// Surprising results from this crate may occur otherwise.
    fn from_io_error_kind(kind: std::io::ErrorKind) -> Self;
}

/// Trait for [`ErrorKind`]s that can be converted to [`std::io::ErrorKind`]. Implementing this trait allows converting to [`std::io::Error`] from [`Error<Self>`][crate::Error]
#[cfg(feature = "std")]
pub trait IntoIoKind: ErrorKind {
    /// Produces a value of [`std::io::ErrorKind`] that corresponds to `self`.
    ///
    /// Except when mapping to an uncategorized error, this method is expected to be consistent with [`FromIoKind::from_io_error_kind`] and [`FromRawOsError::from_raw_os_error`].
    /// Surprising results from this crate may occur otherwise.
    fn into_io_error_kind(self) -> std::io::ErrorKind;
}

#[cfg(feature = "std")]
mod std_impls {
    use super::*;
    impl ErrorKind for std::io::ErrorKind {
        const OTHER: Self = Self::Other;

        fn uncategorized() -> Self {
            Self::Other // We can't emit a properly uncategorized error here, so...
        }
    }

    impl FromIoKind for std::io::ErrorKind {
        fn from_io_error_kind(kind: std::io::ErrorKind) -> Self {
            kind
        }
    }

    impl IntoIoKind for std::io::ErrorKind {
        fn into_io_error_kind(self) -> std::io::ErrorKind {
            self
        }
    }

    impl FromRawOsError for std::io::ErrorKind {
        fn from_raw_os_error(raw: RawOsError) -> Self {
            std::io::Error::from_raw_os_error(raw).kind() // Lol
        }
    }
}
