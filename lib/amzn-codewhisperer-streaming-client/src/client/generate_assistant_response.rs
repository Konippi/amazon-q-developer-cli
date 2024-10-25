// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the
    /// [`GenerateAssistantResponse`](crate::operation::generate_assistant_response::builders::GenerateAssistantResponseFluentBuilder)
    /// operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`conversation_state(ConversationState)`](crate::operation::generate_assistant_response::builders::GenerateAssistantResponseFluentBuilder::conversation_state) / [`set_conversation_state(Option<ConversationState>)`](crate::operation::generate_assistant_response::builders::GenerateAssistantResponseFluentBuilder::set_conversation_state):<br>required: **true**<br>Structure to represent the current state of a chat conversation.<br>
    ///   - [`profile_arn(impl Into<String>)`](crate::operation::generate_assistant_response::builders::GenerateAssistantResponseFluentBuilder::profile_arn) / [`set_profile_arn(Option<String>)`](crate::operation::generate_assistant_response::builders::GenerateAssistantResponseFluentBuilder::set_profile_arn):<br>required: **false**<br>(undocumented)<br>
    /// - On success, responds with
    ///   [`GenerateAssistantResponseOutput`](crate::operation::generate_assistant_response::GenerateAssistantResponseOutput)
    ///   with field(s):
    ///   - [`generate_assistant_response_response(EventReceiver<ChatResponseStream, ChatResponseStreamError>)`](crate::operation::generate_assistant_response::GenerateAssistantResponseOutput::generate_assistant_response_response): Streaming events from UniDirectional Streaming Conversational APIs.
    /// - On failure, responds with [`SdkError<GenerateAssistantResponseError>`](crate::operation::generate_assistant_response::GenerateAssistantResponseError)
    pub fn generate_assistant_response(
        &self,
    ) -> crate::operation::generate_assistant_response::builders::GenerateAssistantResponseFluentBuilder {
        crate::operation::generate_assistant_response::builders::GenerateAssistantResponseFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
