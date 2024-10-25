// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct FileContext {
    #[allow(missing_docs)] // documentation missing in model
    pub left_file_content: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub right_file_content: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub filename: ::std::string::String,
    /// Programming Languages supported by CodeWhisperer
    pub programming_language: crate::types::ProgrammingLanguage,
}
impl FileContext {
    #[allow(missing_docs)] // documentation missing in model
    pub fn left_file_content(&self) -> &str {
        use std::ops::Deref;
        self.left_file_content.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn right_file_content(&self) -> &str {
        use std::ops::Deref;
        self.right_file_content.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn filename(&self) -> &str {
        use std::ops::Deref;
        self.filename.deref()
    }

    /// Programming Languages supported by CodeWhisperer
    pub fn programming_language(&self) -> &crate::types::ProgrammingLanguage {
        &self.programming_language
    }
}
impl ::std::fmt::Debug for FileContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("FileContext");
        formatter.field("left_file_content", &"*** Sensitive Data Redacted ***");
        formatter.field("right_file_content", &"*** Sensitive Data Redacted ***");
        formatter.field("filename", &"*** Sensitive Data Redacted ***");
        formatter.field("programming_language", &self.programming_language);
        formatter.finish()
    }
}
impl FileContext {
    /// Creates a new builder-style object to manufacture
    /// [`FileContext`](crate::types::FileContext).
    pub fn builder() -> crate::types::builders::FileContextBuilder {
        crate::types::builders::FileContextBuilder::default()
    }
}

/// A builder for [`FileContext`](crate::types::FileContext).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct FileContextBuilder {
    pub(crate) left_file_content: ::std::option::Option<::std::string::String>,
    pub(crate) right_file_content: ::std::option::Option<::std::string::String>,
    pub(crate) filename: ::std::option::Option<::std::string::String>,
    pub(crate) programming_language: ::std::option::Option<crate::types::ProgrammingLanguage>,
}
impl FileContextBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn left_file_content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.left_file_content = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_left_file_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.left_file_content = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_left_file_content(&self) -> &::std::option::Option<::std::string::String> {
        &self.left_file_content
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn right_file_content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.right_file_content = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_right_file_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.right_file_content = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_right_file_content(&self) -> &::std::option::Option<::std::string::String> {
        &self.right_file_content
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn filename(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.filename = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_filename(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.filename = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_filename(&self) -> &::std::option::Option<::std::string::String> {
        &self.filename
    }

    /// Programming Languages supported by CodeWhisperer
    /// This field is required.
    pub fn programming_language(mut self, input: crate::types::ProgrammingLanguage) -> Self {
        self.programming_language = ::std::option::Option::Some(input);
        self
    }

    /// Programming Languages supported by CodeWhisperer
    pub fn set_programming_language(mut self, input: ::std::option::Option<crate::types::ProgrammingLanguage>) -> Self {
        self.programming_language = input;
        self
    }

    /// Programming Languages supported by CodeWhisperer
    pub fn get_programming_language(&self) -> &::std::option::Option<crate::types::ProgrammingLanguage> {
        &self.programming_language
    }

    /// Consumes the builder and constructs a [`FileContext`](crate::types::FileContext).
    /// This method will fail if any of the following fields are not set:
    /// - [`left_file_content`](crate::types::builders::FileContextBuilder::left_file_content)
    /// - [`right_file_content`](crate::types::builders::FileContextBuilder::right_file_content)
    /// - [`filename`](crate::types::builders::FileContextBuilder::filename)
    /// - [`programming_language`](crate::types::builders::FileContextBuilder::programming_language)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::FileContext, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::FileContext {
            left_file_content: self.left_file_content.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "left_file_content",
                    "left_file_content was not specified but it is required when building FileContext",
                )
            })?,
            right_file_content: self.right_file_content.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "right_file_content",
                    "right_file_content was not specified but it is required when building FileContext",
                )
            })?,
            filename: self.filename.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "filename",
                    "filename was not specified but it is required when building FileContext",
                )
            })?,
            programming_language: self.programming_language.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "programming_language",
                    "programming_language was not specified but it is required when building FileContext",
                )
            })?,
        })
    }
}
impl ::std::fmt::Debug for FileContextBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("FileContextBuilder");
        formatter.field("left_file_content", &"*** Sensitive Data Redacted ***");
        formatter.field("right_file_content", &"*** Sensitive Data Redacted ***");
        formatter.field("filename", &"*** Sensitive Data Redacted ***");
        formatter.field("programming_language", &self.programming_language);
        formatter.finish()
    }
}
