// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `ValidationExceptionReason`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let validationexceptionreason = unimplemented!();
/// match validationexceptionreason {
///     ValidationExceptionReason::BadRequest => { /* ... */ },
///     ValidationExceptionReason::ContentLengthExceedsThreshold => { /* ... */ },
///     ValidationExceptionReason::InvalidConversationId => { /* ... */ },
///     ValidationExceptionReason::InvalidInput => { /* ... */ },
///     ValidationExceptionReason::PromptInjection => { /* ... */ },
///     ValidationExceptionReason::SessionExpired => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `validationexceptionreason` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ValidationExceptionReason::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ValidationExceptionReason::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ValidationExceptionReason::NewFeature` is defined.
/// Specifically, when `validationexceptionreason` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ValidationExceptionReason::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
/// Reason for ValidationException
#[non_exhaustive]
#[derive(
    ::std::clone::Clone,
    ::std::cmp::Eq,
    ::std::cmp::Ord,
    ::std::cmp::PartialEq,
    ::std::cmp::PartialOrd,
    ::std::fmt::Debug,
    ::std::hash::Hash,
)]
pub enum ValidationExceptionReason {
    #[allow(missing_docs)] // documentation missing in model
    BadRequest,
    #[allow(missing_docs)] // documentation missing in model
    ContentLengthExceedsThreshold,
    #[allow(missing_docs)] // documentation missing in model
    InvalidConversationId,
    #[allow(missing_docs)] // documentation missing in model
    InvalidInput,
    #[allow(missing_docs)] // documentation missing in model
    PromptInjection,
    #[allow(missing_docs)] // documentation missing in model
    SessionExpired,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(
        note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants."
    )]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for ValidationExceptionReason {
    fn from(s: &str) -> Self {
        match s {
            "BAD_REQUEST" => ValidationExceptionReason::BadRequest,
            "CONTENT_LENGTH_EXCEEDS_THRESHOLD" => ValidationExceptionReason::ContentLengthExceedsThreshold,
            "INVALID_CONVERSATION_ID" => ValidationExceptionReason::InvalidConversationId,
            "INVALID_INPUT" => ValidationExceptionReason::InvalidInput,
            "PROMPT_INJECTION" => ValidationExceptionReason::PromptInjection,
            "SESSION_EXPIRED" => ValidationExceptionReason::SessionExpired,
            other => ValidationExceptionReason::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl ::std::str::FromStr for ValidationExceptionReason {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(ValidationExceptionReason::from(s))
    }
}
impl ValidationExceptionReason {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ValidationExceptionReason::BadRequest => "BAD_REQUEST",
            ValidationExceptionReason::ContentLengthExceedsThreshold => "CONTENT_LENGTH_EXCEEDS_THRESHOLD",
            ValidationExceptionReason::InvalidConversationId => "INVALID_CONVERSATION_ID",
            ValidationExceptionReason::InvalidInput => "INVALID_INPUT",
            ValidationExceptionReason::PromptInjection => "PROMPT_INJECTION",
            ValidationExceptionReason::SessionExpired => "SESSION_EXPIRED",
            ValidationExceptionReason::Unknown(value) => value.as_str(),
        }
    }

    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "BAD_REQUEST",
            "CONTENT_LENGTH_EXCEEDS_THRESHOLD",
            "INVALID_CONVERSATION_ID",
            "INVALID_INPUT",
            "PROMPT_INJECTION",
            "SESSION_EXPIRED",
        ]
    }
}
impl ::std::convert::AsRef<str> for ValidationExceptionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl ValidationExceptionReason {
    /// Parses the enum value while disallowing unknown variants.
    ///
    /// Unknown variants will result in an error.
    pub fn try_parse(value: &str) -> ::std::result::Result<Self, crate::error::UnknownVariantError> {
        match Self::from(value) {
            #[allow(deprecated)]
            Self::Unknown(_) => ::std::result::Result::Err(crate::error::UnknownVariantError::new(value)),
            known => Ok(known),
        }
    }
}
impl ::std::fmt::Display for ValidationExceptionReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            ValidationExceptionReason::BadRequest => write!(f, "BAD_REQUEST"),
            ValidationExceptionReason::ContentLengthExceedsThreshold => write!(f, "CONTENT_LENGTH_EXCEEDS_THRESHOLD"),
            ValidationExceptionReason::InvalidConversationId => write!(f, "INVALID_CONVERSATION_ID"),
            ValidationExceptionReason::InvalidInput => write!(f, "INVALID_INPUT"),
            ValidationExceptionReason::PromptInjection => write!(f, "PROMPT_INJECTION"),
            ValidationExceptionReason::SessionExpired => write!(f, "SESSION_EXPIRED"),
            ValidationExceptionReason::Unknown(value) => write!(f, "{}", value),
        }
    }
}
