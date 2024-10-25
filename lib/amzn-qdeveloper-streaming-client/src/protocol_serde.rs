// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<
        ::aws_smithy_runtime_api::client::interceptors::context::Error,
    >,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub(crate) mod shape_generate_code_from_commands;

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_send_message;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() { b"{}" } else { data }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_dry_run_operation_exception;

pub(crate) mod shape_generate_code_from_commands_input;

pub(crate) mod shape_generate_code_from_commands_output;

pub(crate) mod shape_internal_server_error;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_send_message_input;

pub(crate) mod shape_send_message_output;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_validation_exception;

pub fn parse_event_stream_error_metadata(
    payload: &::bytes::Bytes,
) -> Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    // Note: HeaderMap::new() doesn't allocate
    crate::json_errors::parse_error_metadata(payload, &::aws_smithy_runtime_api::http::Headers::new())
}

pub(crate) mod shape_command_input;

pub(crate) mod shape_conversation_state;

pub(crate) mod shape_assistant_response_event;

pub(crate) mod shape_chat_message;

pub(crate) mod shape_code_event;

pub(crate) mod shape_code_reference_event;

pub(crate) mod shape_followup_prompt_event;

pub(crate) mod shape_intents_event;

pub(crate) mod shape_invalid_state_event;

pub(crate) mod shape_message_metadata_event;

pub(crate) mod shape_supplementary_web_links_event;

pub(crate) mod shape_assistant_response_message;

pub(crate) mod shape_user_input_message;

pub(crate) mod shape_followup_prompt;

pub(crate) mod shape_intent_map;

pub(crate) mod shape_reference;

pub(crate) mod shape_references;

pub(crate) mod shape_supplementary_web_link;

pub(crate) mod shape_supplementary_web_links;

pub(crate) mod shape_user_input_message_context;

pub(crate) mod shape_app_studio_state;

pub(crate) mod shape_console_state;

pub(crate) mod shape_diagnostic;

pub(crate) mod shape_editor_state;

pub(crate) mod shape_env_state;

pub(crate) mod shape_git_state;

pub(crate) mod shape_intent_data;

pub(crate) mod shape_shell_state;

pub(crate) mod shape_span;

pub(crate) mod shape_user_settings;

pub(crate) mod shape_cursor_state;

pub(crate) mod shape_environment_variable;

pub(crate) mod shape_intent_data_type;

pub(crate) mod shape_relevant_text_document;

pub(crate) mod shape_runtime_diagnostic;

pub(crate) mod shape_shell_history_entry;

pub(crate) mod shape_text_document;

pub(crate) mod shape_text_document_diagnostic;

pub(crate) mod shape_document_symbol;

pub(crate) mod shape_position;

pub(crate) mod shape_programming_language;

pub(crate) mod shape_range;
