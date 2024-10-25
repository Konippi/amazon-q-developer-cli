// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_metadata_entry(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MetadataEntry,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.key {
        object.key("Key").string(var_1.as_str());
    }
    if let Some(var_2) = &input.value {
        object.key("Value").string(var_2.as_str());
    }
    Ok(())
}
