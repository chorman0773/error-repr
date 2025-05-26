use crate::{
    RawOsError,
    kind::{ErrorKind, FromRawOsError},
};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[derive(Debug)]
enum ErrorPayload {
    Simple,
    RawOsError(RawOsError),
    Message(&'static str),
    #[cfg(feature = "alloc")]
    Error(Box<dyn core::error::Error + Send + Sync + 'static>),
}

/// Primary Error type from this crate.
/// [`Error`] contains a kind and an optional payload, which may be an OS Error Code, a Custom Message, or (with an allocator available) a custom Error object.
#[derive(Debug)]
pub struct Error<K> {
    kind: K,
    payload: ErrorPayload,
    #[cfg(feature = "error-track_caller")]
    #[allow(dead_code)]
    caller_location: &'static core::panic::Location<'static>, // intentionally unused field
}

impl<K: ErrorKind> core::fmt::Display for Error<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.kind, f)?;
        match &self.payload {
            ErrorPayload::Simple => Ok(()),
            ErrorPayload::RawOsError(os) => f.write_fmt(format_args!(" (raw os error {os})")),
            ErrorPayload::Message(m) => f.write_fmt(format_args!(": {m}")),
            #[cfg(feature = "alloc")]
            ErrorPayload::Error(error) => f.write_fmt(format_args!(": {error}")),
        }
    }
}

impl<K> Error<K> {
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    const fn internal_new(kind: K, payload: ErrorPayload) -> Self {
        Self {
            kind,
            payload,
            #[cfg(feature = "error-track_caller")]
            caller_location: core::panic::Location::caller(),
        }
    }

    /// Constructs a new error with a kind, but not payload
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    pub const fn new_simple(kind: K) -> Self {
        Self::internal_new(kind, ErrorPayload::Simple)
    }

    /// Constructs a new error with a kind and a custom message
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    pub const fn new_with_message(kind: K, msg: &'static str) -> Self {
        Self::internal_new(kind, ErrorPayload::Message(msg))
    }

    /// Constructs a new error with a kind and a custom payload
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    #[cfg(feature = "alloc")]
    pub fn new<E: Into<Box<dyn core::error::Error + Send + Sync + 'static>>>(
        kind: K,
        error: E,
    ) -> Self {
        Self::internal_new(kind, ErrorPayload::Error(error.into()))
    }

    /// Returns the error Kind of the Error
    pub fn kind(&self) -> K
    where
        K: Copy,
    {
        self.kind
    }

    /// Returns the raw os error if the error was constructed with one (via. [`Error::from_raw_os_error`])
    pub fn raw_os_error(&self) -> Option<RawOsError> {
        match self.payload {
            ErrorPayload::RawOsError(e) => Some(e),
            _ => None,
        }
    }

    /// Converts `self` into an boxed error if a custom payload is present.
    /// [`Error::into_error`] will return [`Some`] if constructed with a custom payload ([`Error::new`], [`Error::other`], or [`Error::uncategorized`]),
    /// or if constructed with a custom message ([`Error::new_with_message`], [`Error::other_with_message`], or [`Error::uncategorized_with_message`]), and [`None`] otherwise.
    ///
    /// Note that the latter case cannot be downcast.
    #[cfg(feature = "alloc")]
    pub fn into_error(self) -> Option<Box<dyn core::error::Error + Send + Sync + 'static>> {
        match self.payload {
            ErrorPayload::Error(payload) => Some(payload),
            ErrorPayload::Message(m) => Some(m.into()),
            _ => None,
        }
    }

    /// Converts `self` into an boxed error if a custom payload is present.
    /// [`Error::into_inner`] will return [`Some`] if constructed with a custom payload ([`Error::new`], [`Error::other`], or [`Error::uncategorized`]), and [`None`] otherwise.
    #[cfg(feature = "alloc")]
    pub fn into_inner(self) -> Option<Box<dyn core::error::Error + Send + Sync + 'static>> {
        match self.payload {
            ErrorPayload::Error(payload) => Some(payload),
            _ => None,
        }
    }
}

impl<K: ErrorKind> Error<K> {
    /// Constructs a new [`Error`] with no payload that indicates an other error.
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    pub const fn other_simple() -> Self {
        Self::internal_new(K::OTHER, ErrorPayload::Simple)
    }

    /// Constructs a new [`Error`] with a custom message that indicates an other error.
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    pub const fn other_with_message(msg: &'static str) -> Self {
        Self::internal_new(K::OTHER, ErrorPayload::Message(msg))
    }

    /// Constructs a new [`Error`] with a custom payload that indicates an other error.
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    #[cfg(feature = "alloc")]
    pub fn other<E: Into<Box<dyn core::error::Error + Send + Sync + 'static>>>(error: E) -> Self {
        Self::internal_new(K::OTHER, ErrorPayload::Error(error.into()))
    }

    /// Constructs a new [`Error`] with no payload that indicates an uncategorized error.
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    pub fn uncategorized_simple() -> Self {
        Self::internal_new(K::uncategorized(), ErrorPayload::Simple)
    }

    /// Constructs a new [`Error`] with a custom message that indicates an uncategorized error.
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    pub fn uncategorized_with_message(msg: &'static str) -> Self {
        Self::internal_new(K::uncategorized(), ErrorPayload::Message(msg))
    }

    /// Constructs a new [`Error`] with a custom payload that indicates an uncategorized error.
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    #[cfg(feature = "alloc")]
    pub fn uncategorized<E: Into<Box<dyn core::error::Error + Send + Sync + 'static>>>(
        error: E,
    ) -> Self {
        Self::internal_new(K::uncategorized(), ErrorPayload::Error(error.into()))
    }
}

impl<K: FromRawOsError> Error<K> {
    /// Constructs a new [`Error`] that contains an Error created from the OS
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    pub fn from_raw_os_error(error: RawOsError) -> Self {
        Self::internal_new(K::from_raw_os_error(error), ErrorPayload::RawOsError(error))
    }
}

impl<K: ErrorKind> core::error::Error for Error<K> {}

#[cfg(feature = "std")]
impl<K: crate::kind::FromIoKind> From<std::io::ErrorKind> for Error<K> {
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    fn from(kind: std::io::ErrorKind) -> Self {
        Error::new_simple(K::from_io_error_kind(kind))
    }
}

#[cfg(feature = "std")]
impl<K: crate::kind::FromIoKind> From<std::io::Error> for Error<K> {
    #[cfg_attr(feature = "error-track_caller", track_caller)]
    fn from(value: std::io::Error) -> Self {
        let kind = K::from_io_error_kind(value.kind());
        if let Some(code) = value.raw_os_error() {
            Self::internal_new(kind, ErrorPayload::RawOsError(code))
        } else if let Some(data) = value.into_inner() {
            Self::internal_new(kind, ErrorPayload::Error(data))
        } else {
            Self::internal_new(kind, ErrorPayload::Simple)
        }
    }
}

#[cfg(feature = "std")]
impl<K: crate::kind::IntoIoKind> From<Error<K>> for std::io::Error {
    fn from(value: Error<K>) -> Self {
        if let Some(code) = value.raw_os_error() {
            Self::from_raw_os_error(code)
        } else {
            let kind = K::into_io_error_kind(value.kind());

            if let Some(payload) = value.into_error() {
                std::io::Error::new(kind, payload)
            } else {
                std::io::Error::new(kind, "(no context)")
            }
        }
    }
}
