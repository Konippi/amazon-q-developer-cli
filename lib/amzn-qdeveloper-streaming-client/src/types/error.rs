// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::types::error::_access_denied_exception::AccessDeniedError;
pub use crate::types::error::_conflict_exception::ConflictError;
pub use crate::types::error::_dry_run_operation_exception::DryRunOperationError;
pub use crate::types::error::_internal_server_error::InternalServerError;
pub use crate::types::error::_resource_not_found_exception::ResourceNotFoundError;
pub use crate::types::error::_service_quota_exceeded_exception::ServiceQuotaExceededError;
pub use crate::types::error::_throttling_exception::ThrottlingError;
pub use crate::types::error::_validation_exception::ValidationError;

/// Error type for the `ChatResponseStreamError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum ChatResponseStreamError {
    /// This exception is thrown when an unexpected error occurred during the processing of a
    /// request.
    InternalServerError(crate::types::error::InternalServerError),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error
    /// code).
    #[deprecated(
        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-ChatResponseStreamError) for what information is available for the error."
    )]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl ChatResponseStreamError {
    /// Creates the `ChatResponseStreamError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<
            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        >,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `ChatResponseStreamError::Unhandled` variant from an
    /// [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.clone().into(),
            meta: err,
        })
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalServerError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }

    /// Returns `true` if the error kind is `ChatResponseStreamError::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(self, Self::InternalServerError(_))
    }
}
impl ::std::error::Error for ChatResponseStreamError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::InternalServerError(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for ChatResponseStreamError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::InternalServerError(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => {
                if let ::std::option::Option::Some(code) =
                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
                {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            },
        }
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for ChatResponseStreamError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }

    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        match self {
            Self::InternalServerError(inner) => ::std::option::Option::Some(inner.retryable_error_kind()),
            _ => ::std::option::Option::None,
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ChatResponseStreamError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalServerError(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            },
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for ChatResponseStreamError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source,
            meta: meta.unwrap_or_default(),
        })
    }
}
impl ::aws_types::request_id::RequestId for crate::types::error::ChatResponseStreamError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

/// Error type for the `GenerateCodeFromCommandsResponseStreamError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum GenerateCodeFromCommandsResponseStreamError {
    /// This exception is thrown when an unexpected error occurred during the processing of a
    /// request.
    InternalServerError(crate::types::error::InternalServerError),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error
    /// code).
    #[deprecated(
        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GenerateCodeFromCommandsResponseStreamError) for what information is available for the error."
    )]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl GenerateCodeFromCommandsResponseStreamError {
    /// Creates the `GenerateCodeFromCommandsResponseStreamError::Unhandled` variant from any error
    /// type.
    pub fn unhandled(
        err: impl ::std::convert::Into<
            ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        >,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `GenerateCodeFromCommandsResponseStreamError::Unhandled` variant from an
    /// [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.clone().into(),
            meta: err,
        })
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalServerError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }

    /// Returns `true` if the error kind is
    /// `GenerateCodeFromCommandsResponseStreamError::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(self, Self::InternalServerError(_))
    }
}
impl ::std::error::Error for GenerateCodeFromCommandsResponseStreamError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::InternalServerError(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for GenerateCodeFromCommandsResponseStreamError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::InternalServerError(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => {
                if let ::std::option::Option::Some(code) =
                    ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
                {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            },
        }
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for GenerateCodeFromCommandsResponseStreamError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }

    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        match self {
            Self::InternalServerError(inner) => ::std::option::Option::Some(inner.retryable_error_kind()),
            _ => ::std::option::Option::None,
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GenerateCodeFromCommandsResponseStreamError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalServerError(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            },
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for GenerateCodeFromCommandsResponseStreamError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source,
            meta: meta.unwrap_or_default(),
        })
    }
}
impl ::aws_types::request_id::RequestId for crate::types::error::GenerateCodeFromCommandsResponseStreamError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

mod _access_denied_exception;

mod _conflict_exception;

mod _dry_run_operation_exception;

mod _internal_server_error;

mod _resource_not_found_exception;

mod _service_quota_exceeded_exception;

mod _throttling_exception;

mod _validation_exception;

/// Builders
pub mod builders;
