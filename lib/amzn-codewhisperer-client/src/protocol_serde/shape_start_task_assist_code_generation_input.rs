// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_task_assist_code_generation_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_task_assist_code_generation::StartTaskAssistCodeGenerationInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.conversation_state {
        #[allow(unused_mut)]
        let mut object_2 = object.key("conversationState").start_object();
        crate::protocol_serde::shape_conversation_state::ser_conversation_state(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.workspace_state {
        #[allow(unused_mut)]
        let mut object_4 = object.key("workspaceState").start_object();
        crate::protocol_serde::shape_workspace_state::ser_workspace_state(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.task_assist_plan {
        let mut array_6 = object.key("taskAssistPlan").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_task_assist_plan_step::ser_task_assist_plan_step(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}
