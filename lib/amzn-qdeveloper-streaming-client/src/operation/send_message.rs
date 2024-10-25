// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `SendMessage`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SendMessage;
impl SendMessage {
    /// Creates a new `SendMessage`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::send_message::SendMessageInput,
    ) -> ::std::result::Result<
        crate::operation::send_message::SendMessageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_message::SendMessageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::send_message::SendMessageError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(
            runtime_plugins,
            input,
            ::aws_smithy_runtime::client::orchestrator::StopPoint::None,
        )
        .await
        .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::send_message::SendMessageOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::send_message::SendMessageInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
            "qdeveloperstreaming",
            "SendMessage",
            input,
            runtime_plugins,
            stop_point,
        )
        .await
    }

    pub(crate) fn operation_runtime_plugins(
        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        client_config: &crate::config::Config,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());
        runtime_plugins = runtime_plugins.with_client_plugin(crate::auth_plugin::DefaultAuthOptionsPlugin::new(vec![
            ::aws_runtime::auth::sigv4::SCHEME_ID,
        ]));
        if let ::std::option::Option::Some(config_override) = config_override {
            for plugin in config_override.runtime_plugins.iter().cloned() {
                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
            }
            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
                config_override,
                client_config.config.clone(),
                &client_config.runtime_components,
            ));
        }
        runtime_plugins
    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for SendMessage {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("SendMessage");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            SendMessageRequestSerializer,
        ));
        cfg.store_put(
            ::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(SendMessageResponseDeserializer),
        );

        cfg.store_put(
            ::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
                ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
            ),
        );

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::SensitiveOutput);
        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "SendMessage",
            "qdeveloperstreaming",
        ));
        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = false;
        signing_options.normalize_uri_path = true;
        signing_options.payload_override = None;

        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
            signing_options,
            ..::std::default::Default::default()
        });

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(
        &self,
        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        #[allow(unused_mut)]
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new(
            "SendMessage",
        )
        .with_interceptor(
            ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default(),
        )
        .with_interceptor(SendMessageEndpointParamsInterceptor)
        .with_retry_classifier(
            ::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::send_message::SendMessageError,
            >::new(),
        )
        .with_retry_classifier(
            ::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::send_message::SendMessageError,
            >::new(),
        )
        .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
            crate::operation::send_message::SendMessageError,
        >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct SendMessageResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for SendMessageResponseDeserializer {
    fn deserialize_streaming(
        &self,
        response: &mut ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::std::option::Option<::aws_smithy_runtime_api::client::interceptors::context::OutputOrError> {
        #[allow(unused_mut)]
        let mut force_error = false;
        ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));

        // If this is an error, defer to the non-streaming parser
        if (!response.status().is_success() && response.status().as_u16() != 200) || force_error {
            return ::std::option::Option::None;
        }
        ::std::option::Option::Some(crate::protocol_serde::type_erase_result(
            crate::protocol_serde::shape_send_message::de_send_message_http_response(response),
        ))
    }

    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        // For streaming operations, we only hit this case if its an error
        let body = response.body().bytes().expect("body loaded");
        crate::protocol_serde::type_erase_result(crate::protocol_serde::shape_send_message::de_send_message_http_error(
            response.status().as_u16(),
            response.headers(),
            body,
        ))
    }
}
#[derive(Debug)]
struct SendMessageRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for SendMessageRequestSerializer {
    #[allow(
        unused_mut,
        clippy::let_and_return,
        clippy::needless_borrow,
        clippy::useless_conversion
    )]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::orchestrator::HttpRequest,
        ::aws_smithy_runtime_api::box_error::BoxError,
    > {
        let input = input
            .downcast::<crate::operation::send_message::SendMessageInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::send_message::SendMessageInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::send_message::SendMessageInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError>
            {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(
                builder,
                ::http::header::CONTENT_TYPE,
                "application/x-amz-json-1.0",
            );
            builder = _header_serialization_settings.set_default_header(
                builder,
                ::http::header::HeaderName::from_static("x-amz-target"),
                "AmazonQDeveloperStreamingService.SendMessage",
            );
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(
            crate::protocol_serde::shape_send_message::ser_send_message_input(&input)?,
        );
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(
                request_builder,
                ::http::header::CONTENT_LENGTH,
                &content_length,
            );
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct SendMessageEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for SendMessageEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "SendMessageEndpointParamsInterceptor"
    }

    fn read_before_execution(
        &self,
        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
            '_,
            ::aws_smithy_runtime_api::client::interceptors::context::Input,
            ::aws_smithy_runtime_api::client::interceptors::context::Output,
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
        >,
        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let _input = context
            .input()
            .downcast_ref::<SendMessageInput>()
            .ok_or("failed to downcast to SendMessageInput")?;

        let params = crate::config::endpoint::Params::builder().build().map_err(|err| {
            ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new(
                "endpoint params could not be built",
                err,
            )
        })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(
                params,
            ));
        ::std::result::Result::Ok(())
    }
}

// The get_* functions below are generated from JMESPath expressions in the
// operationContextParams trait. They target the operation's input shape.

/// Error type for the `SendMessageError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum SendMessageError {
    /// This exception is thrown when request was denied due to caller exceeding their usage limits
    ServiceQuotaExceededError(crate::types::error::ServiceQuotaExceededError),
    /// This exception is translated to a 204 as it succeeded the IAM Auth.
    DryRunOperationError(crate::types::error::DryRunOperationError),
    /// This exception is thrown when request was denied due to request throttling.
    ThrottlingError(crate::types::error::ThrottlingError),
    /// This exception is thrown when the action to perform could not be completed because the
    /// resource is in a conflicting state.
    ConflictError(crate::types::error::ConflictError),
    /// This exception is thrown when the input fails to satisfy the constraints specified by the
    /// service.
    ValidationError(crate::types::error::ValidationError),
    /// This exception is thrown when an unexpected error occurred during the processing of a
    /// request.
    InternalServerError(crate::types::error::InternalServerError),
    /// This exception is thrown when describing a resource that does not exist.
    ResourceNotFoundError(crate::types::error::ResourceNotFoundError),
    /// This exception is thrown when the user does not have sufficient access to perform this
    /// action.
    AccessDeniedError(crate::types::error::AccessDeniedError),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error
    /// code).
    #[deprecated(
        note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-SendMessageError) for what information is available for the error."
    )]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl SendMessageError {
    /// Creates the `SendMessageError::Unhandled` variant from any error type.
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

    /// Creates the `SendMessageError::Unhandled` variant from an
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
            Self::ServiceQuotaExceededError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DryRunOperationError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ThrottlingError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ConflictError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ValidationError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InternalServerError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ResourceNotFoundError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::AccessDeniedError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }

    /// Returns `true` if the error kind is `SendMessageError::ServiceQuotaExceededError`.
    pub fn is_service_quota_exceeded_error(&self) -> bool {
        matches!(self, Self::ServiceQuotaExceededError(_))
    }

    /// Returns `true` if the error kind is `SendMessageError::DryRunOperationError`.
    pub fn is_dry_run_operation_error(&self) -> bool {
        matches!(self, Self::DryRunOperationError(_))
    }

    /// Returns `true` if the error kind is `SendMessageError::ThrottlingError`.
    pub fn is_throttling_error(&self) -> bool {
        matches!(self, Self::ThrottlingError(_))
    }

    /// Returns `true` if the error kind is `SendMessageError::ConflictError`.
    pub fn is_conflict_error(&self) -> bool {
        matches!(self, Self::ConflictError(_))
    }

    /// Returns `true` if the error kind is `SendMessageError::ValidationError`.
    pub fn is_validation_error(&self) -> bool {
        matches!(self, Self::ValidationError(_))
    }

    /// Returns `true` if the error kind is `SendMessageError::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(self, Self::InternalServerError(_))
    }

    /// Returns `true` if the error kind is `SendMessageError::ResourceNotFoundError`.
    pub fn is_resource_not_found_error(&self) -> bool {
        matches!(self, Self::ResourceNotFoundError(_))
    }

    /// Returns `true` if the error kind is `SendMessageError::AccessDeniedError`.
    pub fn is_access_denied_error(&self) -> bool {
        matches!(self, Self::AccessDeniedError(_))
    }
}
impl ::std::error::Error for SendMessageError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::ServiceQuotaExceededError(_inner) => ::std::option::Option::Some(_inner),
            Self::DryRunOperationError(_inner) => ::std::option::Option::Some(_inner),
            Self::ThrottlingError(_inner) => ::std::option::Option::Some(_inner),
            Self::ConflictError(_inner) => ::std::option::Option::Some(_inner),
            Self::ValidationError(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalServerError(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundError(_inner) => ::std::option::Option::Some(_inner),
            Self::AccessDeniedError(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for SendMessageError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::ServiceQuotaExceededError(_inner) => _inner.fmt(f),
            Self::DryRunOperationError(_inner) => _inner.fmt(f),
            Self::ThrottlingError(_inner) => _inner.fmt(f),
            Self::ConflictError(_inner) => _inner.fmt(f),
            Self::ValidationError(_inner) => _inner.fmt(f),
            Self::InternalServerError(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundError(_inner) => _inner.fmt(f),
            Self::AccessDeniedError(_inner) => _inner.fmt(f),
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
impl ::aws_smithy_types::retry::ProvideErrorKind for SendMessageError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }

    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        match self {
            Self::ThrottlingError(inner) => ::std::option::Option::Some(inner.retryable_error_kind()),
            Self::InternalServerError(inner) => ::std::option::Option::Some(inner.retryable_error_kind()),
            _ => ::std::option::Option::None,
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for SendMessageError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ServiceQuotaExceededError(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            },
            Self::DryRunOperationError(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            },
            Self::ThrottlingError(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ConflictError(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ValidationError(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InternalServerError(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            },
            Self::ResourceNotFoundError(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            },
            Self::AccessDeniedError(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for SendMessageError {
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
impl ::aws_types::request_id::RequestId for crate::operation::send_message::SendMessageError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::send_message::_send_message_input::SendMessageInput;
pub use crate::operation::send_message::_send_message_output::SendMessageOutput;

mod _send_message_input;

mod _send_message_output;

/// Builders
pub mod builders;
