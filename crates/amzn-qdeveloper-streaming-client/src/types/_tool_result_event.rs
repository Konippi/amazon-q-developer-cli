// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ToolResultEvent {
    /// A tool result that contains the results for a tool request that was previously made.
    pub tool_result: ::std::option::Option<crate::types::ToolResult>,
}
impl ToolResultEvent {
    /// A tool result that contains the results for a tool request that was previously made.
    pub fn tool_result(&self) -> ::std::option::Option<&crate::types::ToolResult> {
        self.tool_result.as_ref()
    }
}
impl ToolResultEvent {
    /// Creates a new builder-style object to manufacture
    /// [`ToolResultEvent`](crate::types::ToolResultEvent).
    pub fn builder() -> crate::types::builders::ToolResultEventBuilder {
        crate::types::builders::ToolResultEventBuilder::default()
    }
}

/// A builder for [`ToolResultEvent`](crate::types::ToolResultEvent).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ToolResultEventBuilder {
    pub(crate) tool_result: ::std::option::Option<crate::types::ToolResult>,
}
impl ToolResultEventBuilder {
    /// A tool result that contains the results for a tool request that was previously made.
    pub fn tool_result(mut self, input: crate::types::ToolResult) -> Self {
        self.tool_result = ::std::option::Option::Some(input);
        self
    }

    /// A tool result that contains the results for a tool request that was previously made.
    pub fn set_tool_result(mut self, input: ::std::option::Option<crate::types::ToolResult>) -> Self {
        self.tool_result = input;
        self
    }

    /// A tool result that contains the results for a tool request that was previously made.
    pub fn get_tool_result(&self) -> &::std::option::Option<crate::types::ToolResult> {
        &self.tool_result
    }

    /// Consumes the builder and constructs a [`ToolResultEvent`](crate::types::ToolResultEvent).
    pub fn build(self) -> crate::types::ToolResultEvent {
        crate::types::ToolResultEvent {
            tool_result: self.tool_result,
        }
    }
}
