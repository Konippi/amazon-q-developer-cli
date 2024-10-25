// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_tags_for_resource_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
    crate::operation::list_tags_for_resource::ListTagsForResourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder =
        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
            .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => {
            crate::operation::list_tags_for_resource::ListTagsForResourceError::InternalServerError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                    output =
                        crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::internal_server_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?
                };
                tmp
            })
        },
        "ThrottlingException" => crate::operation::list_tags_for_resource::ListTagsForResourceError::ThrottlingError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingErrorBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::list_tags_for_resource::ListTagsForResourceError::ValidationError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationErrorBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ResourceNotFoundError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundErrorBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_not_found_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?
                };
                tmp
            })
        },
        "AccessDeniedException" => {
            crate::operation::list_tags_for_resource::ListTagsForResourceError::AccessDeniedError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedErrorBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::access_denied_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?
                };
                tmp
            })
        },
        _ => crate::operation::list_tags_for_resource::ListTagsForResourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_tags_for_resource_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_tags_for_resource::ListTagsForResourceOutput,
    crate::operation::list_tags_for_resource::ListTagsForResourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder::default();
        output = crate::protocol_serde::shape_list_tags_for_resource::de_list_tags_for_resource(_response_body, output)
            .map_err(crate::operation::list_tags_for_resource::ListTagsForResourceError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_list_tags_for_resource_input(
    input: &crate::operation::list_tags_for_resource::ListTagsForResourceInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_tags_for_resource_input::ser_list_tags_for_resource_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_list_tags_for_resource(
    value: &[u8],
    mut builder: crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder,
) -> Result<
    crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "tags" => {
                    builder = builder.set_tags(crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?);
                },
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                    format!("expected object key or end object, found: {:?}", other),
                ));
            },
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
