// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the
    /// [`UnlockServiceLinkedRole`](crate::operation::unlock_service_linked_role::builders::UnlockServiceLinkedRoleFluentBuilder)
    /// operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`role_arn(impl Into<String>)`](crate::operation::unlock_service_linked_role::builders::UnlockServiceLinkedRoleFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::unlock_service_linked_role::builders::UnlockServiceLinkedRoleFluentBuilder::set_role_arn):<br>required: **true**<br>(undocumented)<br>
    ///   - [`deletion_status(DeletionStatus)`](crate::operation::unlock_service_linked_role::builders::UnlockServiceLinkedRoleFluentBuilder::deletion_status) / [`set_deletion_status(Option<DeletionStatus>)`](crate::operation::unlock_service_linked_role::builders::UnlockServiceLinkedRoleFluentBuilder::set_deletion_status):<br>required: **false**<br>(undocumented)<br>
    /// - On success, responds with
    ///   [`UnlockServiceLinkedRoleOutput`](crate::operation::unlock_service_linked_role::UnlockServiceLinkedRoleOutput)
    /// - On failure, responds with
    ///   [`SdkError<UnlockServiceLinkedRoleError>`](crate::operation::unlock_service_linked_role::UnlockServiceLinkedRoleError)
    pub fn unlock_service_linked_role(
        &self,
    ) -> crate::operation::unlock_service_linked_role::builders::UnlockServiceLinkedRoleFluentBuilder {
        crate::operation::unlock_service_linked_role::builders::UnlockServiceLinkedRoleFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
