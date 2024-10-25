// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CliCommand {
    #[allow(missing_docs)] // documentation missing in model
    pub command: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub description: ::std::option::Option<::std::string::String>,
}
impl CliCommand {
    #[allow(missing_docs)] // documentation missing in model
    pub fn command(&self) -> ::std::option::Option<&str> {
        self.command.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl ::std::fmt::Debug for CliCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CliCommand");
        formatter.field("command", &"*** Sensitive Data Redacted ***");
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl CliCommand {
    /// Creates a new builder-style object to manufacture [`CliCommand`](crate::types::CliCommand).
    pub fn builder() -> crate::types::builders::CliCommandBuilder {
        crate::types::builders::CliCommandBuilder::default()
    }
}

/// A builder for [`CliCommand`](crate::types::CliCommand).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct CliCommandBuilder {
    pub(crate) command: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
}
impl CliCommandBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn command(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.command = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_command(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.command = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_command(&self) -> &::std::option::Option<::std::string::String> {
        &self.command
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

    /// Consumes the builder and constructs a [`CliCommand`](crate::types::CliCommand).
    pub fn build(self) -> crate::types::CliCommand {
        crate::types::CliCommand {
            command: self.command,
            description: self.description,
        }
    }
}
impl ::std::fmt::Debug for CliCommandBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CliCommandBuilder");
        formatter.field("command", &"*** Sensitive Data Redacted ***");
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
