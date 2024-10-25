// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(Debug)]
pub struct GenerateCodeFromCommandsResponseStreamUnmarshaller;

impl GenerateCodeFromCommandsResponseStreamUnmarshaller {
    pub fn new() -> Self {
        GenerateCodeFromCommandsResponseStreamUnmarshaller
    }
}
impl ::aws_smithy_eventstream::frame::UnmarshallMessage for GenerateCodeFromCommandsResponseStreamUnmarshaller {
    type Error = crate::types::error::GenerateCodeFromCommandsResponseStreamError;
    type Output = crate::types::GenerateCodeFromCommandsResponseStream;

    fn unmarshall(
        &self,
        message: &::aws_smithy_types::event_stream::Message,
    ) -> std::result::Result<
        ::aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>,
        ::aws_smithy_eventstream::error::Error,
    > {
        let response_headers = ::aws_smithy_eventstream::smithy::parse_response_headers(message)?;
        match response_headers.message_type.as_str() {
            "event" => match response_headers.smithy_type.as_str() {
                "codeEvent" => {
                    let parsed = crate::protocol_serde::shape_code_event::de_code_event_payload(&message.payload()[..])
                        .map_err(|err| {
                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                "failed to unmarshall CodeEvent: {}",
                                err
                            ))
                        })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::GenerateCodeFromCommandsResponseStream::CodeEvent(parsed),
                    ))
                },
                _unknown_variant => Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                    crate::types::GenerateCodeFromCommandsResponseStream::Unknown,
                )),
            },
            "exception" => {
                let generic = match crate::protocol_serde::parse_event_stream_error_metadata(message.payload()) {
                    Ok(builder) => builder.build(),
                    Err(err) => {
                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::GenerateCodeFromCommandsResponseStreamError::unhandled(err),
                        ));
                    },
                };
                if response_headers.smithy_type.as_str() == "error" {
                    let mut builder = crate::types::error::builders::InternalServerErrorBuilder::default();
                    builder = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(
                        &message.payload()[..],
                        builder,
                    )
                    .map_err(|err| {
                        ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                            "failed to unmarshall error: {}",
                            err
                        ))
                    })?;
                    builder.set_meta(Some(generic));
                    return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                        crate::types::error::GenerateCodeFromCommandsResponseStreamError::InternalServerError(
                            crate::serde_util::internal_server_error_correct_errors(builder)
                                .build()
                                .map_err(|err| {
                                    ::aws_smithy_eventstream::error::Error::unmarshalling(format!("{}", err))
                                })?,
                        ),
                    ));
                }
                Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                    crate::types::error::GenerateCodeFromCommandsResponseStreamError::generic(generic),
                ))
            },
            value => {
                return Err(::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                    "unrecognized :message-type: {}",
                    value
                )));
            },
        }
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct ChatResponseStreamUnmarshaller;

impl ChatResponseStreamUnmarshaller {
    pub fn new() -> Self {
        ChatResponseStreamUnmarshaller
    }
}
impl ::aws_smithy_eventstream::frame::UnmarshallMessage for ChatResponseStreamUnmarshaller {
    type Error = crate::types::error::ChatResponseStreamError;
    type Output = crate::types::ChatResponseStream;

    fn unmarshall(
        &self,
        message: &::aws_smithy_types::event_stream::Message,
    ) -> std::result::Result<
        ::aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>,
        ::aws_smithy_eventstream::error::Error,
    > {
        let response_headers = ::aws_smithy_eventstream::smithy::parse_response_headers(message)?;
        match response_headers.message_type.as_str() {
            "event" => match response_headers.smithy_type.as_str() {
                "messageMetadataEvent" => {
                    let parsed =
                        crate::protocol_serde::shape_message_metadata_event::de_message_metadata_event_payload(
                            &message.payload()[..],
                        )
                        .map_err(|err| {
                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                "failed to unmarshall MessageMetadataEvent: {}",
                                err
                            ))
                        })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::ChatResponseStream::MessageMetadataEvent(parsed),
                    ))
                },
                "assistantResponseEvent" => {
                    let parsed =
                        crate::protocol_serde::shape_assistant_response_event::de_assistant_response_event_payload(
                            &message.payload()[..],
                        )
                        .map_err(|err| {
                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                "failed to unmarshall AssistantResponseEvent: {}",
                                err
                            ))
                        })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::ChatResponseStream::AssistantResponseEvent(parsed),
                    ))
                },
                "codeReferenceEvent" => {
                    let parsed = crate::protocol_serde::shape_code_reference_event::de_code_reference_event_payload(
                        &message.payload()[..],
                    )
                    .map_err(|err| {
                        ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                            "failed to unmarshall CodeReferenceEvent: {}",
                            err
                        ))
                    })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::ChatResponseStream::CodeReferenceEvent(parsed),
                    ))
                },
                "supplementaryWebLinksEvent" => {
                    let parsed =
                        crate::protocol_serde::shape_supplementary_web_links_event::de_supplementary_web_links_event_payload(&message.payload()[..])
                            .map_err(|err| {
                                ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                    "failed to unmarshall SupplementaryWebLinksEvent: {}",
                                    err
                                ))
                            })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::ChatResponseStream::SupplementaryWebLinksEvent(parsed),
                    ))
                },
                "followupPromptEvent" => {
                    let parsed = crate::protocol_serde::shape_followup_prompt_event::de_followup_prompt_event_payload(
                        &message.payload()[..],
                    )
                    .map_err(|err| {
                        ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                            "failed to unmarshall FollowupPromptEvent: {}",
                            err
                        ))
                    })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::ChatResponseStream::FollowupPromptEvent(parsed),
                    ))
                },
                "codeEvent" => {
                    let parsed = crate::protocol_serde::shape_code_event::de_code_event_payload(&message.payload()[..])
                        .map_err(|err| {
                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                "failed to unmarshall CodeEvent: {}",
                                err
                            ))
                        })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::ChatResponseStream::CodeEvent(parsed),
                    ))
                },
                "intentsEvent" => {
                    let parsed =
                        crate::protocol_serde::shape_intents_event::de_intents_event_payload(&message.payload()[..])
                            .map_err(|err| {
                                ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                    "failed to unmarshall IntentsEvent: {}",
                                    err
                                ))
                            })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::ChatResponseStream::IntentsEvent(parsed),
                    ))
                },
                "invalidStateEvent" => {
                    let parsed = crate::protocol_serde::shape_invalid_state_event::de_invalid_state_event_payload(
                        &message.payload()[..],
                    )
                    .map_err(|err| {
                        ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                            "failed to unmarshall InvalidStateEvent: {}",
                            err
                        ))
                    })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::ChatResponseStream::InvalidStateEvent(parsed),
                    ))
                },
                _unknown_variant => Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                    crate::types::ChatResponseStream::Unknown,
                )),
            },
            "exception" => {
                let generic = match crate::protocol_serde::parse_event_stream_error_metadata(message.payload()) {
                    Ok(builder) => builder.build(),
                    Err(err) => {
                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::ChatResponseStreamError::unhandled(err),
                        ));
                    },
                };
                if response_headers.smithy_type.as_str() == "error" {
                    let mut builder = crate::types::error::builders::InternalServerErrorBuilder::default();
                    builder = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(
                        &message.payload()[..],
                        builder,
                    )
                    .map_err(|err| {
                        ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                            "failed to unmarshall error: {}",
                            err
                        ))
                    })?;
                    builder.set_meta(Some(generic));
                    return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                        crate::types::error::ChatResponseStreamError::InternalServerError(
                            crate::serde_util::internal_server_error_correct_errors(builder)
                                .build()
                                .map_err(|err| {
                                    ::aws_smithy_eventstream::error::Error::unmarshalling(format!("{}", err))
                                })?,
                        ),
                    ));
                }
                Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                    crate::types::error::ChatResponseStreamError::generic(generic),
                ))
            },
            value => {
                return Err(::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                    "unrecognized :message-type: {}",
                    value
                )));
            },
        }
    }
}
