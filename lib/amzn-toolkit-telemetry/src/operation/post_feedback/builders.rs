// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::post_feedback::_post_feedback_input::PostFeedbackInputBuilder;
pub use crate::operation::post_feedback::_post_feedback_output::PostFeedbackOutputBuilder;

impl PostFeedbackInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::post_feedback::PostFeedbackOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::post_feedback::PostFeedbackError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.post_feedback();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PostFeedback`.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PostFeedbackFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::post_feedback::builders::PostFeedbackInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::post_feedback::PostFeedbackOutput,
        crate::operation::post_feedback::PostFeedbackError,
    > for PostFeedbackFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::post_feedback::PostFeedbackOutput,
            crate::operation::post_feedback::PostFeedbackError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PostFeedbackFluentBuilder {
    /// Creates a new `PostFeedback`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }

    /// Access the PostFeedback as a reference.
    pub fn as_input(&self) -> &crate::operation::post_feedback::builders::PostFeedbackInputBuilder {
        &self.inner
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::post_feedback::PostFeedbackOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::post_feedback::PostFeedbackError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::post_feedback::PostFeedback::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::post_feedback::PostFeedback::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::post_feedback::PostFeedbackOutput,
        crate::operation::post_feedback::PostFeedbackError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }

    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn aws_product(mut self, input: crate::types::AwsProduct) -> Self {
        self.inner = self.inner.aws_product(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_aws_product(mut self, input: ::std::option::Option<crate::types::AwsProduct>) -> Self {
        self.inner = self.inner.set_aws_product(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_aws_product(&self) -> &::std::option::Option<crate::types::AwsProduct> {
        self.inner.get_aws_product()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn aws_product_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_product_version(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_aws_product_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_product_version(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_aws_product_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_product_version()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn os(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.os(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_os(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_os(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_os(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_os()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn os_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.os_version(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_os_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_os_version(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_os_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_os_version()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn parent_product(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.parent_product(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_parent_product(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_parent_product(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_parent_product(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_parent_product()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn parent_product_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.parent_product_version(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_parent_product_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_parent_product_version(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_parent_product_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_parent_product_version()
    }

    /// Appends an item to `Metadata`.
    ///
    /// To override the contents of this collection use [`set_metadata`](Self::set_metadata).
    #[allow(missing_docs)] // documentation missing in model
    pub fn metadata(mut self, input: crate::types::MetadataEntry) -> Self {
        self.inner = self.inner.metadata(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_metadata(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MetadataEntry>>) -> Self {
        self.inner = self.inner.set_metadata(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_metadata(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MetadataEntry>> {
        self.inner.get_metadata()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn sentiment(mut self, input: crate::types::Sentiment) -> Self {
        self.inner = self.inner.sentiment(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_sentiment(mut self, input: ::std::option::Option<crate::types::Sentiment>) -> Self {
        self.inner = self.inner.set_sentiment(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_sentiment(&self) -> &::std::option::Option<crate::types::Sentiment> {
        self.inner.get_sentiment()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.comment(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_comment(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_comment(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_comment()
    }
}
