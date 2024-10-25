// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CustomizationSummary {
    #[allow(missing_docs)] // documentation missing in model
    pub arn: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub version: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub customization_name: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub description: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub status: crate::types::CustomizationStatus,
    #[allow(missing_docs)] // documentation missing in model
    pub updated_at: ::aws_smithy_types::DateTime,
}
impl CustomizationSummary {
    #[allow(missing_docs)] // documentation missing in model
    pub fn arn(&self) -> &str {
        use std::ops::Deref;
        self.arn.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn version(&self) -> ::std::option::Option<i64> {
        self.version
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn customization_name(&self) -> &str {
        use std::ops::Deref;
        self.customization_name.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn status(&self) -> &crate::types::CustomizationStatus {
        &self.status
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn updated_at(&self) -> &::aws_smithy_types::DateTime {
        &self.updated_at
    }
}
impl CustomizationSummary {
    /// Creates a new builder-style object to manufacture
    /// [`CustomizationSummary`](crate::types::CustomizationSummary).
    pub fn builder() -> crate::types::builders::CustomizationSummaryBuilder {
        crate::types::builders::CustomizationSummaryBuilder::default()
    }
}

/// A builder for [`CustomizationSummary`](crate::types::CustomizationSummary).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CustomizationSummaryBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<i64>,
    pub(crate) customization_name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::CustomizationStatus>,
    pub(crate) updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl CustomizationSummaryBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
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

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn customization_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.customization_name = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_customization_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.customization_name = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_customization_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.customization_name
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn status(mut self, input: crate::types::CustomizationStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::CustomizationStatus>) -> Self {
        self.status = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_status(&self) -> &::std::option::Option<crate::types::CustomizationStatus> {
        &self.status
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_at = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated_at = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated_at
    }

    /// Consumes the builder and constructs a
    /// [`CustomizationSummary`](crate::types::CustomizationSummary). This method will fail if
    /// any of the following fields are not set:
    /// - [`arn`](crate::types::builders::CustomizationSummaryBuilder::arn)
    /// - [`customization_name`](crate::types::builders::CustomizationSummaryBuilder::customization_name)
    /// - [`status`](crate::types::builders::CustomizationSummaryBuilder::status)
    /// - [`updated_at`](crate::types::builders::CustomizationSummaryBuilder::updated_at)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::CustomizationSummary, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::types::CustomizationSummary {
            arn: self.arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "arn",
                    "arn was not specified but it is required when building CustomizationSummary",
                )
            })?,
            version: self.version,
            customization_name: self.customization_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "customization_name",
                    "customization_name was not specified but it is required when building CustomizationSummary",
                )
            })?,
            description: self.description,
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building CustomizationSummary",
                )
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "updated_at",
                    "updated_at was not specified but it is required when building CustomizationSummary",
                )
            })?,
        })
    }
}
