// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SubscriptionMetadata {
    #[allow(missing_docs)] // documentation missing in model
    pub status: ::std::option::Option<::std::string::String>,
}
impl SubscriptionMetadata {
    #[allow(missing_docs)] // documentation missing in model
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
}
impl SubscriptionMetadata {
    /// Creates a new builder-style object to manufacture
    /// [`SubscriptionMetadata`](crate::types::SubscriptionMetadata).
    pub fn builder() -> crate::types::builders::SubscriptionMetadataBuilder {
        crate::types::builders::SubscriptionMetadataBuilder::default()
    }
}

/// A builder for [`SubscriptionMetadata`](crate::types::SubscriptionMetadata).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SubscriptionMetadataBuilder {
    pub(crate) status: ::std::option::Option<::std::string::String>,
}
impl SubscriptionMetadataBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_status(&self) -> &::std::option::Option<::std::string::String> {
        &self.status
    }

    /// Consumes the builder and constructs a
    /// [`SubscriptionMetadata`](crate::types::SubscriptionMetadata).
    pub fn build(self) -> crate::types::SubscriptionMetadata {
        crate::types::SubscriptionMetadata { status: self.status }
    }
}
