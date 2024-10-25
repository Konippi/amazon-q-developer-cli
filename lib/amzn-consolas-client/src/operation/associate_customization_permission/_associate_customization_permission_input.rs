// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateCustomizationPermissionInput {
    #[allow(missing_docs)] // documentation missing in model
    pub identifier: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub permission: ::std::option::Option<crate::types::CustomizationPermission>,
}
impl AssociateCustomizationPermissionInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn identifier(&self) -> ::std::option::Option<&str> {
        self.identifier.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn permission(&self) -> ::std::option::Option<&crate::types::CustomizationPermission> {
        self.permission.as_ref()
    }
}
impl AssociateCustomizationPermissionInput {
    /// Creates a new builder-style object to manufacture
    /// [`AssociateCustomizationPermissionInput`](crate::operation::associate_customization_permission::AssociateCustomizationPermissionInput).
    pub fn builder()
    -> crate::operation::associate_customization_permission::builders::AssociateCustomizationPermissionInputBuilder
    {
        crate::operation::associate_customization_permission::builders::AssociateCustomizationPermissionInputBuilder::default()
    }
}

/// A builder for
/// [`AssociateCustomizationPermissionInput`](crate::operation::associate_customization_permission::AssociateCustomizationPermissionInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AssociateCustomizationPermissionInputBuilder {
    pub(crate) identifier: ::std::option::Option<::std::string::String>,
    pub(crate) permission: ::std::option::Option<crate::types::CustomizationPermission>,
}
impl AssociateCustomizationPermissionInputBuilder {
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
    pub fn permission(mut self, input: crate::types::CustomizationPermission) -> Self {
        self.permission = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_permission(mut self, input: ::std::option::Option<crate::types::CustomizationPermission>) -> Self {
        self.permission = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_permission(&self) -> &::std::option::Option<crate::types::CustomizationPermission> {
        &self.permission
    }

    /// Consumes the builder and constructs a
    /// [`AssociateCustomizationPermissionInput`](crate::operation::associate_customization_permission::AssociateCustomizationPermissionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_customization_permission::AssociateCustomizationPermissionInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::associate_customization_permission::AssociateCustomizationPermissionInput {
                identifier: self.identifier,
                permission: self.permission,
            },
        )
    }
}
