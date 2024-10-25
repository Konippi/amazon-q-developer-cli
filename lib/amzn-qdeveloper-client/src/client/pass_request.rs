// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the
    /// [`PassRequest`](crate::operation::pass_request::builders::PassRequestFluentBuilder)
    /// operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`extension_fas_policy_path(impl Into<String>)`](crate::operation::pass_request::builders::PassRequestFluentBuilder::extension_fas_policy_path) / [`set_extension_fas_policy_path(Option<String>)`](crate::operation::pass_request::builders::PassRequestFluentBuilder::set_extension_fas_policy_path):<br>required: **false**<br>(undocumented)<br>
    ///   - [`extension_kms_key_arn(impl Into<String>)`](crate::operation::pass_request::builders::PassRequestFluentBuilder::extension_kms_key_arn) / [`set_extension_kms_key_arn(Option<String>)`](crate::operation::pass_request::builders::PassRequestFluentBuilder::set_extension_kms_key_arn):<br>required: **false**<br>(undocumented)<br>
    ///   - [`tools(Tool)`](crate::operation::pass_request::builders::PassRequestFluentBuilder::tools) / [`set_tools(Option<Vec::<Tool>>)`](crate::operation::pass_request::builders::PassRequestFluentBuilder::set_tools):<br>required: **false**<br>(undocumented)<br>
    /// - On success, responds with
    ///   [`PassRequestOutput`](crate::operation::pass_request::PassRequestOutput) with field(s):
    ///   - [`encrypted_extension_fas_creds(Option<String>)`](crate::operation::pass_request::PassRequestOutput::encrypted_extension_fas_creds): (undocumented)
    ///   - [`encrypted_tools_fas_creds(Option<Vec::<EncryptedToolFasCreds>>)`](crate::operation::pass_request::PassRequestOutput::encrypted_tools_fas_creds): (undocumented)
    /// - On failure, responds with
    ///   [`SdkError<PassRequestError>`](crate::operation::pass_request::PassRequestError)
    pub fn pass_request(&self) -> crate::operation::pass_request::builders::PassRequestFluentBuilder {
        crate::operation::pass_request::builders::PassRequestFluentBuilder::new(self.handle.clone())
    }
}
