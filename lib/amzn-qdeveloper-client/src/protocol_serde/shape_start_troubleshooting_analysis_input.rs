// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_troubleshooting_analysis_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_troubleshooting_analysis::StartTroubleshootingAnalysisInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.error_detail {
        #[allow(unused_mut)]
        let mut object_2 = object.key("errorDetail").start_object();
        crate::protocol_serde::shape_error_detail::ser_error_detail(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
