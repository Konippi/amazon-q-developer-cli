// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the
    /// [`ListFeatureEvaluations`](crate::operation::list_feature_evaluations::builders::ListFeatureEvaluationsFluentBuilder)
    /// operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_context(UserContext)`](crate::operation::list_feature_evaluations::builders::ListFeatureEvaluationsFluentBuilder::user_context) / [`set_user_context(Option<UserContext>)`](crate::operation::list_feature_evaluations::builders::ListFeatureEvaluationsFluentBuilder::set_user_context):<br>required: **true**<br>(undocumented)<br>
    ///   - [`profile_arn(impl Into<String>)`](crate::operation::list_feature_evaluations::builders::ListFeatureEvaluationsFluentBuilder::profile_arn) / [`set_profile_arn(Option<String>)`](crate::operation::list_feature_evaluations::builders::ListFeatureEvaluationsFluentBuilder::set_profile_arn):<br>required: **false**<br>(undocumented)<br>
    /// - On success, responds with
    ///   [`ListFeatureEvaluationsOutput`](crate::operation::list_feature_evaluations::ListFeatureEvaluationsOutput)
    ///   with field(s):
    ///   - [`feature_evaluations(Vec::<FeatureEvaluation>)`](crate::operation::list_feature_evaluations::ListFeatureEvaluationsOutput::feature_evaluations): (undocumented)
    /// - On failure, responds with
    ///   [`SdkError<ListFeatureEvaluationsError>`](crate::operation::list_feature_evaluations::ListFeatureEvaluationsError)
    pub fn list_feature_evaluations(
        &self,
    ) -> crate::operation::list_feature_evaluations::builders::ListFeatureEvaluationsFluentBuilder {
        crate::operation::list_feature_evaluations::builders::ListFeatureEvaluationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
