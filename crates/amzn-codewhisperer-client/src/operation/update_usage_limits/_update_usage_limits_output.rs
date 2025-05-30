// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateUsageLimitsOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub status: crate::types::UsageLimitUpdateRequestStatus,
    #[allow(missing_docs)] // documentation missing in model
    pub approved_limit: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub remaining_requests_this_month: ::std::option::Option<i32>,
    _request_id: Option<String>,
}
impl UpdateUsageLimitsOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn status(&self) -> &crate::types::UsageLimitUpdateRequestStatus {
        &self.status
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn approved_limit(&self) -> ::std::option::Option<i64> {
        self.approved_limit
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn remaining_requests_this_month(&self) -> ::std::option::Option<i32> {
        self.remaining_requests_this_month
    }
}
impl ::aws_types::request_id::RequestId for UpdateUsageLimitsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateUsageLimitsOutput {
    /// Creates a new builder-style object to manufacture
    /// [`UpdateUsageLimitsOutput`](crate::operation::update_usage_limits::UpdateUsageLimitsOutput).
    pub fn builder() -> crate::operation::update_usage_limits::builders::UpdateUsageLimitsOutputBuilder {
        crate::operation::update_usage_limits::builders::UpdateUsageLimitsOutputBuilder::default()
    }
}

/// A builder for
/// [`UpdateUsageLimitsOutput`](crate::operation::update_usage_limits::UpdateUsageLimitsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateUsageLimitsOutputBuilder {
    pub(crate) status: ::std::option::Option<crate::types::UsageLimitUpdateRequestStatus>,
    pub(crate) approved_limit: ::std::option::Option<i64>,
    pub(crate) remaining_requests_this_month: ::std::option::Option<i32>,
    _request_id: Option<String>,
}
impl UpdateUsageLimitsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn status(mut self, input: crate::types::UsageLimitUpdateRequestStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::UsageLimitUpdateRequestStatus>) -> Self {
        self.status = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_status(&self) -> &::std::option::Option<crate::types::UsageLimitUpdateRequestStatus> {
        &self.status
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn approved_limit(mut self, input: i64) -> Self {
        self.approved_limit = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_approved_limit(mut self, input: ::std::option::Option<i64>) -> Self {
        self.approved_limit = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_approved_limit(&self) -> &::std::option::Option<i64> {
        &self.approved_limit
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn remaining_requests_this_month(mut self, input: i32) -> Self {
        self.remaining_requests_this_month = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_remaining_requests_this_month(mut self, input: ::std::option::Option<i32>) -> Self {
        self.remaining_requests_this_month = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_remaining_requests_this_month(&self) -> &::std::option::Option<i32> {
        &self.remaining_requests_this_month
    }

    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }

    /// Consumes the builder and constructs a
    /// [`UpdateUsageLimitsOutput`](crate::operation::update_usage_limits::UpdateUsageLimitsOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](crate::operation::update_usage_limits::builders::UpdateUsageLimitsOutputBuilder::status)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_usage_limits::UpdateUsageLimitsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_usage_limits::UpdateUsageLimitsOutput {
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building UpdateUsageLimitsOutput",
                )
            })?,
            approved_limit: self.approved_limit,
            remaining_requests_this_month: self.remaining_requests_this_month,
            _request_id: self._request_id,
        })
    }
}
