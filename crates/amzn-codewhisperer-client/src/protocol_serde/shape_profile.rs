// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_profile<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::Profile>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<
        Item = Result<
            ::aws_smithy_json::deserialize::Token<'a>,
            ::aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ProfileBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "arn" => {
                                builder = builder.set_arn(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            },
                            "identityDetails" => {
                                builder = builder.set_identity_details(
                                    crate::protocol_serde::shape_identity_details::de_identity_details(tokens)?,
                                );
                            },
                            "profileName" => {
                                builder = builder.set_profile_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            },
                            "description" => {
                                builder = builder.set_description(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            },
                            "referenceTrackerConfiguration" => {
                                builder = builder.set_reference_tracker_configuration(
                                crate::protocol_serde::shape_reference_tracker_configuration::de_reference_tracker_configuration(tokens)?,
                            );
                            },
                            "kmsKeyArn" => {
                                builder = builder.set_kms_key_arn(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            },
                            "activeFunctionalities" => {
                                builder = builder.set_active_functionalities(
                                crate::protocol_serde::shape_active_functionality_list::de_active_functionality_list(tokens)?,
                            );
                            },
                            "status" => {
                                builder = builder.set_status(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| {
                                            s.to_unescaped().map(|u| crate::types::ProfileStatus::from(u.as_ref()))
                                        })
                                        .transpose()?,
                                );
                            },
                            "errorDetails" => {
                                builder = builder.set_error_details(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            },
                            "resourcePolicy" => {
                                builder = builder.set_resource_policy(
                                    crate::protocol_serde::shape_resource_policy::de_resource_policy(tokens)?,
                                );
                            },
                            "profileType" => {
                                builder = builder.set_profile_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| crate::types::ProfileType::from(u.as_ref())))
                                        .transpose()?,
                                );
                            },
                            "optInFeatures" => {
                                builder = builder.set_opt_in_features(
                                    crate::protocol_serde::shape_opt_in_features::de_opt_in_features(tokens)?,
                                );
                            },
                            "permissionUpdateRequired" => {
                                builder = builder.set_permission_update_required(
                                    ::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?,
                                );
                            },
                            "applicationProperties" => {
                                builder = builder.set_application_properties(
                                crate::protocol_serde::shape_application_properties_list::de_application_properties_list(tokens)?,
                            );
                            },
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            format!("expected object key or end object, found: {:?}", other),
                        ));
                    },
                }
            }
            Ok(Some(
                crate::serde_util::profile_correct_errors(builder)
                    .build()
                    .map_err(|err| {
                        ::aws_smithy_json::deserialize::error::DeserializeError::custom_source(
                            "Response was invalid",
                            err,
                        )
                    })?,
            ))
        },
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
