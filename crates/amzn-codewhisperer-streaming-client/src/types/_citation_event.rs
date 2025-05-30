// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Streaming response event for citations
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CitationEvent {
    /// The position or the range of the response text to be cited
    pub target: crate::types::CitationTarget,
    /// The text inside the citation '1' in \[1\]
    pub citation_text: ::std::option::Option<::std::string::String>,
    /// The link to the document being cited
    pub citation_link: ::std::string::String,
}
impl CitationEvent {
    /// The position or the range of the response text to be cited
    pub fn target(&self) -> &crate::types::CitationTarget {
        &self.target
    }

    /// The text inside the citation '1' in \[1\]
    pub fn citation_text(&self) -> ::std::option::Option<&str> {
        self.citation_text.as_deref()
    }

    /// The link to the document being cited
    pub fn citation_link(&self) -> &str {
        use std::ops::Deref;
        self.citation_link.deref()
    }
}
impl ::std::fmt::Debug for CitationEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CitationEvent");
        formatter.field("target", &self.target);
        formatter.field("citation_text", &"*** Sensitive Data Redacted ***");
        formatter.field("citation_link", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl CitationEvent {
    /// Creates a new builder-style object to manufacture
    /// [`CitationEvent`](crate::types::CitationEvent).
    pub fn builder() -> crate::types::builders::CitationEventBuilder {
        crate::types::builders::CitationEventBuilder::default()
    }
}

/// A builder for [`CitationEvent`](crate::types::CitationEvent).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct CitationEventBuilder {
    pub(crate) target: ::std::option::Option<crate::types::CitationTarget>,
    pub(crate) citation_text: ::std::option::Option<::std::string::String>,
    pub(crate) citation_link: ::std::option::Option<::std::string::String>,
}
impl CitationEventBuilder {
    /// The position or the range of the response text to be cited
    /// This field is required.
    pub fn target(mut self, input: crate::types::CitationTarget) -> Self {
        self.target = ::std::option::Option::Some(input);
        self
    }

    /// The position or the range of the response text to be cited
    pub fn set_target(mut self, input: ::std::option::Option<crate::types::CitationTarget>) -> Self {
        self.target = input;
        self
    }

    /// The position or the range of the response text to be cited
    pub fn get_target(&self) -> &::std::option::Option<crate::types::CitationTarget> {
        &self.target
    }

    /// The text inside the citation '1' in \[1\]
    pub fn citation_text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.citation_text = ::std::option::Option::Some(input.into());
        self
    }

    /// The text inside the citation '1' in \[1\]
    pub fn set_citation_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.citation_text = input;
        self
    }

    /// The text inside the citation '1' in \[1\]
    pub fn get_citation_text(&self) -> &::std::option::Option<::std::string::String> {
        &self.citation_text
    }

    /// The link to the document being cited
    /// This field is required.
    pub fn citation_link(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.citation_link = ::std::option::Option::Some(input.into());
        self
    }

    /// The link to the document being cited
    pub fn set_citation_link(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.citation_link = input;
        self
    }

    /// The link to the document being cited
    pub fn get_citation_link(&self) -> &::std::option::Option<::std::string::String> {
        &self.citation_link
    }

    /// Consumes the builder and constructs a [`CitationEvent`](crate::types::CitationEvent).
    /// This method will fail if any of the following fields are not set:
    /// - [`target`](crate::types::builders::CitationEventBuilder::target)
    /// - [`citation_link`](crate::types::builders::CitationEventBuilder::citation_link)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::CitationEvent, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::CitationEvent {
            target: self.target.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "target",
                    "target was not specified but it is required when building CitationEvent",
                )
            })?,
            citation_text: self.citation_text,
            citation_link: self.citation_link.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "citation_link",
                    "citation_link was not specified but it is required when building CitationEvent",
                )
            })?,
        })
    }
}
impl ::std::fmt::Debug for CitationEventBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CitationEventBuilder");
        formatter.field("target", &self.target);
        formatter.field("citation_text", &"*** Sensitive Data Redacted ***");
        formatter.field("citation_link", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
