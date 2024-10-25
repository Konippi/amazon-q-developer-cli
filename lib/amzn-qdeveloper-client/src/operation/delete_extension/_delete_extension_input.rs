// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteExtensionInput {
    #[allow(missing_docs)] // documentation missing in model
    pub extension_id: ::std::option::Option<::std::string::String>,
}
impl DeleteExtensionInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn extension_id(&self) -> ::std::option::Option<&str> {
        self.extension_id.as_deref()
    }
}
impl DeleteExtensionInput {
    /// Creates a new builder-style object to manufacture
    /// [`DeleteExtensionInput`](crate::operation::delete_extension::DeleteExtensionInput).
    pub fn builder() -> crate::operation::delete_extension::builders::DeleteExtensionInputBuilder {
        crate::operation::delete_extension::builders::DeleteExtensionInputBuilder::default()
    }
}

/// A builder for
/// [`DeleteExtensionInput`](crate::operation::delete_extension::DeleteExtensionInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteExtensionInputBuilder {
    pub(crate) extension_id: ::std::option::Option<::std::string::String>,
}
impl DeleteExtensionInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn extension_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.extension_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_extension_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.extension_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_extension_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.extension_id
    }

    /// Consumes the builder and constructs a
    /// [`DeleteExtensionInput`](crate::operation::delete_extension::DeleteExtensionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_extension::DeleteExtensionInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_extension::DeleteExtensionInput {
            extension_id: self.extension_id,
        })
    }
}
