// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the
    /// [`GetTaskAssistCodeGeneration`](crate::operation::get_task_assist_code_generation::builders::GetTaskAssistCodeGenerationFluentBuilder)
    /// operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`conversation_id(impl Into<String>)`](crate::operation::get_task_assist_code_generation::builders::GetTaskAssistCodeGenerationFluentBuilder::conversation_id) / [`set_conversation_id(Option<String>)`](crate::operation::get_task_assist_code_generation::builders::GetTaskAssistCodeGenerationFluentBuilder::set_conversation_id):<br>required: **true**<br>ID which represents a multi-turn conversation<br>
    ///   - [`code_generation_id(impl Into<String>)`](crate::operation::get_task_assist_code_generation::builders::GetTaskAssistCodeGenerationFluentBuilder::code_generation_id) / [`set_code_generation_id(Option<String>)`](crate::operation::get_task_assist_code_generation::builders::GetTaskAssistCodeGenerationFluentBuilder::set_code_generation_id):<br>required: **true**<br>ID which represents a single code generation in a conversation<br>
    ///   - [`profile_arn(impl Into<String>)`](crate::operation::get_task_assist_code_generation::builders::GetTaskAssistCodeGenerationFluentBuilder::profile_arn) / [`set_profile_arn(Option<String>)`](crate::operation::get_task_assist_code_generation::builders::GetTaskAssistCodeGenerationFluentBuilder::set_profile_arn):<br>required: **false**<br>(undocumented)<br>
    /// - On success, responds with [`GetTaskAssistCodeGenerationOutput`](crate::operation::get_task_assist_code_generation::GetTaskAssistCodeGenerationOutput) with field(s):
    ///   - [`conversation_id(String)`](crate::operation::get_task_assist_code_generation::GetTaskAssistCodeGenerationOutput::conversation_id): ID which represents a multi-turn conversation
    ///   - [`code_generation_status(CodeGenerationStatus)`](crate::operation::get_task_assist_code_generation::GetTaskAssistCodeGenerationOutput::code_generation_status): (undocumented)
    ///   - [`code_generation_status_detail(Option<String>)`](crate::operation::get_task_assist_code_generation::GetTaskAssistCodeGenerationOutput::code_generation_status_detail): Detailed message about the code generation status
    ///   - [`code_generation_remaining_iteration_count(Option<i32>)`](crate::operation::get_task_assist_code_generation::GetTaskAssistCodeGenerationOutput::code_generation_remaining_iteration_count): (undocumented)
    ///   - [`code_generation_total_iteration_count(Option<i32>)`](crate::operation::get_task_assist_code_generation::GetTaskAssistCodeGenerationOutput::code_generation_total_iteration_count): (undocumented)
    /// - On failure, responds with [`SdkError<GetTaskAssistCodeGenerationError>`](crate::operation::get_task_assist_code_generation::GetTaskAssistCodeGenerationError)
    pub fn get_task_assist_code_generation(
        &self,
    ) -> crate::operation::get_task_assist_code_generation::builders::GetTaskAssistCodeGenerationFluentBuilder {
        crate::operation::get_task_assist_code_generation::builders::GetTaskAssistCodeGenerationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
