// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct PassRequestInput {
    #[allow(missing_docs)] // documentation missing in model
    pub extension_fas_policy_path: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub extension_kms_key_arn: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub tools: ::std::option::Option<::std::vec::Vec<crate::types::Tool>>,
}
impl PassRequestInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn extension_fas_policy_path(&self) -> ::std::option::Option<&str> {
        self.extension_fas_policy_path.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn extension_kms_key_arn(&self) -> ::std::option::Option<&str> {
        self.extension_kms_key_arn.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.tools.is_none()`.
    pub fn tools(&self) -> &[crate::types::Tool] {
        self.tools.as_deref().unwrap_or_default()
    }
}
impl ::std::fmt::Debug for PassRequestInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("PassRequestInput");
        formatter.field("extension_fas_policy_path", &"*** Sensitive Data Redacted ***");
        formatter.field("extension_kms_key_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("tools", &self.tools);
        formatter.finish()
    }
}
impl PassRequestInput {
    /// Creates a new builder-style object to manufacture
    /// [`PassRequestInput`](crate::operation::pass_request::PassRequestInput).
    pub fn builder() -> crate::operation::pass_request::builders::PassRequestInputBuilder {
        crate::operation::pass_request::builders::PassRequestInputBuilder::default()
    }
}

/// A builder for [`PassRequestInput`](crate::operation::pass_request::PassRequestInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct PassRequestInputBuilder {
    pub(crate) extension_fas_policy_path: ::std::option::Option<::std::string::String>,
    pub(crate) extension_kms_key_arn: ::std::option::Option<::std::string::String>,
    pub(crate) tools: ::std::option::Option<::std::vec::Vec<crate::types::Tool>>,
}
impl PassRequestInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn extension_fas_policy_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.extension_fas_policy_path = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_extension_fas_policy_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.extension_fas_policy_path = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_extension_fas_policy_path(&self) -> &::std::option::Option<::std::string::String> {
        &self.extension_fas_policy_path
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn extension_kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.extension_kms_key_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_extension_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.extension_kms_key_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_extension_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.extension_kms_key_arn
    }

    /// Appends an item to `tools`.
    ///
    /// To override the contents of this collection use [`set_tools`](Self::set_tools).
    pub fn tools(mut self, input: crate::types::Tool) -> Self {
        let mut v = self.tools.unwrap_or_default();
        v.push(input);
        self.tools = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_tools(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tool>>) -> Self {
        self.tools = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_tools(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tool>> {
        &self.tools
    }

    /// Consumes the builder and constructs a
    /// [`PassRequestInput`](crate::operation::pass_request::PassRequestInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::pass_request::PassRequestInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::pass_request::PassRequestInput {
            extension_fas_policy_path: self.extension_fas_policy_path,
            extension_kms_key_arn: self.extension_kms_key_arn,
            tools: self.tools,
        })
    }
}
impl ::std::fmt::Debug for PassRequestInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("PassRequestInputBuilder");
        formatter.field("extension_fas_policy_path", &"*** Sensitive Data Redacted ***");
        formatter.field("extension_kms_key_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("tools", &self.tools);
        formatter.finish()
    }
}
