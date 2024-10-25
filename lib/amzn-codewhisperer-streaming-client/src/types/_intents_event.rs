// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Streaming Response Event for Intents
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IntentsEvent {
    /// A map of Intent objects
    pub intents: ::std::option::Option<
        ::std::collections::HashMap<
            crate::types::IntentType,
            ::std::collections::HashMap<::std::string::String, crate::types::IntentDataType>,
        >,
    >,
}
impl IntentsEvent {
    /// A map of Intent objects
    pub fn intents(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<
            crate::types::IntentType,
            ::std::collections::HashMap<::std::string::String, crate::types::IntentDataType>,
        >,
    > {
        self.intents.as_ref()
    }
}
impl IntentsEvent {
    /// Creates a new builder-style object to manufacture
    /// [`IntentsEvent`](crate::types::IntentsEvent).
    pub fn builder() -> crate::types::builders::IntentsEventBuilder {
        crate::types::builders::IntentsEventBuilder::default()
    }
}

/// A builder for [`IntentsEvent`](crate::types::IntentsEvent).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct IntentsEventBuilder {
    pub(crate) intents: ::std::option::Option<
        ::std::collections::HashMap<
            crate::types::IntentType,
            ::std::collections::HashMap<::std::string::String, crate::types::IntentDataType>,
        >,
    >,
}
impl IntentsEventBuilder {
    /// Adds a key-value pair to `intents`.
    ///
    /// To override the contents of this collection use [`set_intents`](Self::set_intents).
    ///
    /// A map of Intent objects
    pub fn intents(
        mut self,
        k: crate::types::IntentType,
        v: ::std::collections::HashMap<::std::string::String, crate::types::IntentDataType>,
    ) -> Self {
        let mut hash_map = self.intents.unwrap_or_default();
        hash_map.insert(k, v);
        self.intents = ::std::option::Option::Some(hash_map);
        self
    }

    /// A map of Intent objects
    pub fn set_intents(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<
                crate::types::IntentType,
                ::std::collections::HashMap<::std::string::String, crate::types::IntentDataType>,
            >,
        >,
    ) -> Self {
        self.intents = input;
        self
    }

    /// A map of Intent objects
    pub fn get_intents(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<
            crate::types::IntentType,
            ::std::collections::HashMap<::std::string::String, crate::types::IntentDataType>,
        >,
    > {
        &self.intents
    }

    /// Consumes the builder and constructs a [`IntentsEvent`](crate::types::IntentsEvent).
    pub fn build(self) -> crate::types::IntentsEvent {
        crate::types::IntentsEvent { intents: self.intents }
    }
}
