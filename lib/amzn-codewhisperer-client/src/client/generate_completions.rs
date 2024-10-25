// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the
    /// [`GenerateCompletions`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder)
    /// operation. This operation supports pagination; See
    /// [`into_paginator()`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::into_paginator).
    ///
    ///
    /// - The fluent builder is configurable:
    ///   - [`file_context(FileContext)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::file_context) / [`set_file_context(Option<FileContext>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::set_file_context):<br>required: **true**<br>(undocumented)<br>
    ///   - [`max_results(i32)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::set_max_results):<br>required: **false**<br>(undocumented)<br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::set_next_token):<br>required: **false**<br>(undocumented)<br>
    ///   - [`reference_tracker_configuration(ReferenceTrackerConfiguration)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::reference_tracker_configuration) / [`set_reference_tracker_configuration(Option<ReferenceTrackerConfiguration>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::set_reference_tracker_configuration):<br>required: **false**<br>(undocumented)<br>
    ///   - [`supplemental_contexts(SupplementalContext)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::supplemental_contexts) / [`set_supplemental_contexts(Option<Vec::<SupplementalContext>>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::set_supplemental_contexts):<br>required: **false**<br>(undocumented)<br>
    ///   - [`customization_arn(impl Into<String>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::customization_arn) / [`set_customization_arn(Option<String>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::set_customization_arn):<br>required: **false**<br>(undocumented)<br>
    ///   - [`opt_out_preference(OptOutPreference)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::opt_out_preference) / [`set_opt_out_preference(Option<OptOutPreference>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::set_opt_out_preference):<br>required: **false**<br>(undocumented)<br>
    ///   - [`user_context(UserContext)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::user_context) / [`set_user_context(Option<UserContext>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::set_user_context):<br>required: **false**<br>(undocumented)<br>
    ///   - [`profile_arn(impl Into<String>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::profile_arn) / [`set_profile_arn(Option<String>)`](crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::set_profile_arn):<br>required: **false**<br>(undocumented)<br>
    /// - On success, responds with
    ///   [`GenerateCompletionsOutput`](crate::operation::generate_completions::GenerateCompletionsOutput)
    ///   with field(s):
    ///   - [`completions(Option<Vec::<Completion>>)`](crate::operation::generate_completions::GenerateCompletionsOutput::completions): (undocumented)
    ///   - [`next_token(Option<String>)`](crate::operation::generate_completions::GenerateCompletionsOutput::next_token): (undocumented)
    /// - On failure, responds with
    ///   [`SdkError<GenerateCompletionsError>`](crate::operation::generate_completions::GenerateCompletionsError)
    pub fn generate_completions(
        &self,
    ) -> crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder {
        crate::operation::generate_completions::builders::GenerateCompletionsFluentBuilder::new(self.handle.clone())
    }
}
