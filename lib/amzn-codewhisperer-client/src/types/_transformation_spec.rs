// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransformationSpec {
    #[allow(missing_docs)] // documentation missing in model
    pub transformation_type: ::std::option::Option<crate::types::TransformationType>,
    #[allow(missing_docs)] // documentation missing in model
    pub source: ::std::option::Option<crate::types::TransformationProjectState>,
    #[allow(missing_docs)] // documentation missing in model
    pub target: ::std::option::Option<crate::types::TransformationProjectState>,
}
impl TransformationSpec {
    #[allow(missing_docs)] // documentation missing in model
    pub fn transformation_type(&self) -> ::std::option::Option<&crate::types::TransformationType> {
        self.transformation_type.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn source(&self) -> ::std::option::Option<&crate::types::TransformationProjectState> {
        self.source.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn target(&self) -> ::std::option::Option<&crate::types::TransformationProjectState> {
        self.target.as_ref()
    }
}
impl TransformationSpec {
    /// Creates a new builder-style object to manufacture
    /// [`TransformationSpec`](crate::types::TransformationSpec).
    pub fn builder() -> crate::types::builders::TransformationSpecBuilder {
        crate::types::builders::TransformationSpecBuilder::default()
    }
}

/// A builder for [`TransformationSpec`](crate::types::TransformationSpec).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct TransformationSpecBuilder {
    pub(crate) transformation_type: ::std::option::Option<crate::types::TransformationType>,
    pub(crate) source: ::std::option::Option<crate::types::TransformationProjectState>,
    pub(crate) target: ::std::option::Option<crate::types::TransformationProjectState>,
}
impl TransformationSpecBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn transformation_type(mut self, input: crate::types::TransformationType) -> Self {
        self.transformation_type = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_transformation_type(mut self, input: ::std::option::Option<crate::types::TransformationType>) -> Self {
        self.transformation_type = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_transformation_type(&self) -> &::std::option::Option<crate::types::TransformationType> {
        &self.transformation_type
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn source(mut self, input: crate::types::TransformationProjectState) -> Self {
        self.source = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::TransformationProjectState>) -> Self {
        self.source = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_source(&self) -> &::std::option::Option<crate::types::TransformationProjectState> {
        &self.source
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn target(mut self, input: crate::types::TransformationProjectState) -> Self {
        self.target = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_target(mut self, input: ::std::option::Option<crate::types::TransformationProjectState>) -> Self {
        self.target = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_target(&self) -> &::std::option::Option<crate::types::TransformationProjectState> {
        &self.target
    }

    /// Consumes the builder and constructs a
    /// [`TransformationSpec`](crate::types::TransformationSpec).
    pub fn build(self) -> crate::types::TransformationSpec {
        crate::types::TransformationSpec {
            transformation_type: self.transformation_type,
            source: self.source,
            target: self.target,
        }
    }
}
