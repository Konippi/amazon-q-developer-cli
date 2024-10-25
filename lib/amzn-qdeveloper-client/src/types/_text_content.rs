// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct TextContent {
    #[allow(missing_docs)] // documentation missing in model
    pub body: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub references: ::std::option::Option<::std::vec::Vec<crate::types::NellyUrl>>,
}
impl TextContent {
    #[allow(missing_docs)] // documentation missing in model
    pub fn body(&self) -> &str {
        use std::ops::Deref;
        self.body.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.references.is_none()`.
    pub fn references(&self) -> &[crate::types::NellyUrl] {
        self.references.as_deref().unwrap_or_default()
    }
}
impl ::std::fmt::Debug for TextContent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("TextContent");
        formatter.field("body", &"*** Sensitive Data Redacted ***");
        formatter.field("references", &self.references);
        formatter.finish()
    }
}
impl TextContent {
    /// Creates a new builder-style object to manufacture
    /// [`TextContent`](crate::types::TextContent).
    pub fn builder() -> crate::types::builders::TextContentBuilder {
        crate::types::builders::TextContentBuilder::default()
    }
}

/// A builder for [`TextContent`](crate::types::TextContent).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct TextContentBuilder {
    pub(crate) body: ::std::option::Option<::std::string::String>,
    pub(crate) references: ::std::option::Option<::std::vec::Vec<crate::types::NellyUrl>>,
}
impl TextContentBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn body(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.body = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_body(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.body = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_body(&self) -> &::std::option::Option<::std::string::String> {
        &self.body
    }

    /// Appends an item to `references`.
    ///
    /// To override the contents of this collection use [`set_references`](Self::set_references).
    pub fn references(mut self, input: crate::types::NellyUrl) -> Self {
        let mut v = self.references.unwrap_or_default();
        v.push(input);
        self.references = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_references(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::NellyUrl>>) -> Self {
        self.references = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_references(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::NellyUrl>> {
        &self.references
    }

    /// Consumes the builder and constructs a [`TextContent`](crate::types::TextContent).
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](crate::types::builders::TextContentBuilder::body)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::TextContent, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::TextContent {
            body: self.body.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "body",
                    "body was not specified but it is required when building TextContent",
                )
            })?,
            references: self.references,
        })
    }
}
impl ::std::fmt::Debug for TextContentBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("TextContentBuilder");
        formatter.field("body", &"*** Sensitive Data Redacted ***");
        formatter.field("references", &self.references);
        formatter.finish()
    }
}
