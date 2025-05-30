// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the
    /// [`UpdateProfile`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder)
    /// operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`profile_arn(impl Into<String>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::profile_arn) / [`set_profile_arn(Option<String>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::set_profile_arn):<br>required: **true**<br>(undocumented)<br>
    ///   - [`identity_source(IdentitySource)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::identity_source) / [`set_identity_source(Option<IdentitySource>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::set_identity_source):<br>required: **false**<br>(undocumented)<br>
    ///   - [`profile_name(impl Into<String>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::profile_name) / [`set_profile_name(Option<String>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::set_profile_name):<br>required: **false**<br>(undocumented)<br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::set_description):<br>required: **false**<br>(undocumented)<br>
    ///   - [`reference_tracker_configuration(ReferenceTrackerConfiguration)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::reference_tracker_configuration) / [`set_reference_tracker_configuration(Option<ReferenceTrackerConfiguration>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::set_reference_tracker_configuration):<br>required: **false**<br>(undocumented)<br>
    ///   - [`active_functionalities(FunctionalityName)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::active_functionalities) / [`set_active_functionalities(Option<Vec::<FunctionalityName>>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::set_active_functionalities):<br>required: **false**<br>(undocumented)<br>
    ///   - [`kms_key_arn(impl Into<String>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::kms_key_arn) / [`set_kms_key_arn(Option<String>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::set_kms_key_arn):<br>required: **false**<br>(undocumented)<br>
    ///   - [`resource_policy(ResourcePolicy)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::resource_policy) / [`set_resource_policy(Option<ResourcePolicy>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::set_resource_policy):<br>required: **false**<br>(undocumented)<br>
    ///   - [`target_profile_type(ProfileType)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::target_profile_type) / [`set_target_profile_type(Option<ProfileType>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::set_target_profile_type):<br>required: **false**<br>(undocumented)<br>
    ///   - [`opt_in_features(OptInFeatures)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::opt_in_features) / [`set_opt_in_features(Option<OptInFeatures>)`](crate::operation::update_profile::builders::UpdateProfileFluentBuilder::set_opt_in_features):<br>required: **false**<br>(undocumented)<br>
    /// - On success, responds with
    ///   [`UpdateProfileOutput`](crate::operation::update_profile::UpdateProfileOutput) with
    ///   field(s):
    ///   - [`profile_arn(String)`](crate::operation::update_profile::UpdateProfileOutput::profile_arn): (undocumented)
    /// - On failure, responds with
    ///   [`SdkError<UpdateProfileError>`](crate::operation::update_profile::UpdateProfileError)
    pub fn update_profile(&self) -> crate::operation::update_profile::builders::UpdateProfileFluentBuilder {
        crate::operation::update_profile::builders::UpdateProfileFluentBuilder::new(self.handle.clone())
    }
}
