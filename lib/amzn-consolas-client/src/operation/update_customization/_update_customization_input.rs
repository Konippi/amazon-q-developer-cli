// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateCustomizationInput {
    #[allow(missing_docs)] // documentation missing in model
    pub identifier: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub operation: ::std::option::Option<crate::types::UpdateOperation>,
    #[allow(missing_docs)] // documentation missing in model
    pub client_token: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub data_reference: ::std::option::Option<crate::types::DataReference>,
    #[allow(missing_docs)] // documentation missing in model
    pub version: ::std::option::Option<i64>,
}
impl UpdateCustomizationInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn identifier(&self) -> ::std::option::Option<&str> {
        self.identifier.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn operation(&self) -> ::std::option::Option<&crate::types::UpdateOperation> {
        self.operation.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn data_reference(&self) -> ::std::option::Option<&crate::types::DataReference> {
        self.data_reference.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn version(&self) -> ::std::option::Option<i64> {
        self.version
    }
}
impl UpdateCustomizationInput {
    /// Creates a new builder-style object to manufacture
    /// [`UpdateCustomizationInput`](crate::operation::update_customization::UpdateCustomizationInput).
    pub fn builder() -> crate::operation::update_customization::builders::UpdateCustomizationInputBuilder {
        crate::operation::update_customization::builders::UpdateCustomizationInputBuilder::default()
    }
}

/// A builder for
/// [`UpdateCustomizationInput`](crate::operation::update_customization::UpdateCustomizationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateCustomizationInputBuilder {
    pub(crate) identifier: ::std::option::Option<::std::string::String>,
    pub(crate) operation: ::std::option::Option<crate::types::UpdateOperation>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) data_reference: ::std::option::Option<crate::types::DataReference>,
    pub(crate) version: ::std::option::Option<i64>,
}
impl UpdateCustomizationInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.identifier = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.identifier = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.identifier
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn operation(mut self, input: crate::types::UpdateOperation) -> Self {
        self.operation = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_operation(mut self, input: ::std::option::Option<crate::types::UpdateOperation>) -> Self {
        self.operation = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_operation(&self) -> &::std::option::Option<crate::types::UpdateOperation> {
        &self.operation
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn data_reference(mut self, input: crate::types::DataReference) -> Self {
        self.data_reference = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_data_reference(mut self, input: ::std::option::Option<crate::types::DataReference>) -> Self {
        self.data_reference = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_data_reference(&self) -> &::std::option::Option<crate::types::DataReference> {
        &self.data_reference
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn version(mut self, input: i64) -> Self {
        self.version = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.version = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_version(&self) -> &::std::option::Option<i64> {
        &self.version
    }

    /// Consumes the builder and constructs a
    /// [`UpdateCustomizationInput`](crate::operation::update_customization::UpdateCustomizationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_customization::UpdateCustomizationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_customization::UpdateCustomizationInput {
            identifier: self.identifier,
            operation: self.operation,
            client_token: self.client_token,
            data_reference: self.data_reference,
            version: self.version,
        })
    }
}
