// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct Completion {
    #[allow(missing_docs)] // documentation missing in model
    pub content: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub references: ::std::option::Option<::std::vec::Vec<crate::types::Reference>>,
    #[allow(missing_docs)] // documentation missing in model
    pub most_relevant_missing_imports: ::std::option::Option<::std::vec::Vec<crate::types::Import>>,
}
impl Completion {
    #[allow(missing_docs)] // documentation missing in model
    pub fn content(&self) -> &str {
        use std::ops::Deref;
        self.content.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.references.is_none()`.
    pub fn references(&self) -> &[crate::types::Reference] {
        self.references.as_deref().unwrap_or_default()
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.most_relevant_missing_imports.is_none()`.
    pub fn most_relevant_missing_imports(&self) -> &[crate::types::Import] {
        self.most_relevant_missing_imports.as_deref().unwrap_or_default()
    }
}
impl ::std::fmt::Debug for Completion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("Completion");
        formatter.field("content", &"*** Sensitive Data Redacted ***");
        formatter.field("references", &self.references);
        formatter.field("most_relevant_missing_imports", &self.most_relevant_missing_imports);
        formatter.finish()
    }
}
impl Completion {
    /// Creates a new builder-style object to manufacture [`Completion`](crate::types::Completion).
    pub fn builder() -> crate::types::builders::CompletionBuilder {
        crate::types::builders::CompletionBuilder::default()
    }
}

/// A builder for [`Completion`](crate::types::Completion).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct CompletionBuilder {
    pub(crate) content: ::std::option::Option<::std::string::String>,
    pub(crate) references: ::std::option::Option<::std::vec::Vec<crate::types::Reference>>,
    pub(crate) most_relevant_missing_imports: ::std::option::Option<::std::vec::Vec<crate::types::Import>>,
}
impl CompletionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_content(&self) -> &::std::option::Option<::std::string::String> {
        &self.content
    }

    /// Appends an item to `references`.
    ///
    /// To override the contents of this collection use [`set_references`](Self::set_references).
    pub fn references(mut self, input: crate::types::Reference) -> Self {
        let mut v = self.references.unwrap_or_default();
        v.push(input);
        self.references = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_references(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Reference>>) -> Self {
        self.references = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_references(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Reference>> {
        &self.references
    }

    /// Appends an item to `most_relevant_missing_imports`.
    ///
    /// To override the contents of this collection use
    /// [`set_most_relevant_missing_imports`](Self::set_most_relevant_missing_imports).
    pub fn most_relevant_missing_imports(mut self, input: crate::types::Import) -> Self {
        let mut v = self.most_relevant_missing_imports.unwrap_or_default();
        v.push(input);
        self.most_relevant_missing_imports = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_most_relevant_missing_imports(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Import>>,
    ) -> Self {
        self.most_relevant_missing_imports = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_most_relevant_missing_imports(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Import>> {
        &self.most_relevant_missing_imports
    }

    /// Consumes the builder and constructs a [`Completion`](crate::types::Completion).
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](crate::types::builders::CompletionBuilder::content)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::Completion, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Completion {
            content: self.content.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "content",
                    "content was not specified but it is required when building Completion",
                )
            })?,
            references: self.references,
            most_relevant_missing_imports: self.most_relevant_missing_imports,
        })
    }
}
impl ::std::fmt::Debug for CompletionBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CompletionBuilder");
        formatter.field("content", &"*** Sensitive Data Redacted ***");
        formatter.field("references", &self.references);
        formatter.field("most_relevant_missing_imports", &self.most_relevant_missing_imports);
        formatter.finish()
    }
}
