// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_start_troubleshooting_resolution_explanation_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationOutput,
    crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => {
            crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::ResourceNotFoundError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundErrorBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(
                            crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                        )?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_not_found_exception_correct_errors(output).build().map_err(
                        crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                    )?
                };
                tmp
            })
        }
        "InternalServerException" => {
            crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::InternalServerError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                        .map_err(
                            crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                        )?;
                    let output = output.meta(generic);
                    crate::serde_util::internal_server_exception_correct_errors(output).build().map_err(
                        crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                    )?
                };
                tmp
            })
        }
        "AccessDeniedException" => {
            crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::AccessDeniedError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedErrorBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                        .map_err(
                            crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                        )?;
                    let output = output.meta(generic);
                    crate::serde_util::access_denied_exception_correct_errors(output).build().map_err(
                        crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                    )?
                };
                tmp
            })
        }
        "ConflictException" => {
            crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::ConflictError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConflictErrorBuilder::default();
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output).map_err(
                        crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::conflict_exception_correct_errors(output).build().map_err(
                        crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                    )?
                };
                tmp
            })
        }
        "ValidationException" => {
            crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::ValidationError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationErrorBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(
                        crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::validation_exception_correct_errors(output).build().map_err(
                        crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                    )?
                };
                tmp
            })
        }
        "ServiceQuotaExceededException" => {
            crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::ServiceQuotaExceededError(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::types::error::builders::ServiceQuotaExceededErrorBuilder::default();
                        output = crate::protocol_serde::shape_service_quota_exceeded_exception::de_service_quota_exceeded_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(
                            crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                        )?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "ThrottlingException" => {
            crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::ThrottlingError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingErrorBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output).map_err(
                        crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    crate::serde_util::throttling_exception_correct_errors(output).build().map_err(
                        crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::unhandled,
                    )?
                };
                tmp
            })
        }
        _ => crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_troubleshooting_resolution_explanation_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationOutput,
    crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::start_troubleshooting_resolution_explanation::builders::StartTroubleshootingResolutionExplanationOutputBuilder::default(
            );
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_start_troubleshooting_resolution_explanation_input(
    input: &crate::operation::start_troubleshooting_resolution_explanation::StartTroubleshootingResolutionExplanationInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_start_troubleshooting_resolution_explanation_input::ser_start_troubleshooting_resolution_explanation_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
