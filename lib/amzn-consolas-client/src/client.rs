// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) conf: crate::Config,
    #[allow(dead_code)] // unused when a service does not provide any operations
    pub(crate) runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
}

/// Client for AWS CodeWhisperer
///
/// Client for invoking operations on AWS CodeWhisperer. Each operation on AWS CodeWhisperer is a
/// method on this this struct. `.send()` MUST be invoked on the generated operations to dispatch
/// the request to the service.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct Client {
    handle: ::std::sync::Arc<Handle>,
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    ///
    /// # Panics
    ///
    /// This method will panic in the following cases:
    ///
    /// - Retries or timeouts are enabled without a `sleep_impl` configured.
    /// - Identity caching is enabled without a `sleep_impl` and `time_source` configured.
    /// - No `behavior_version` is provided.
    ///
    /// The panic message for each of these will have instructions on how to resolve them.
    #[track_caller]
    pub fn from_conf(conf: crate::Config) -> Self {
        let handle = Handle {
            conf: conf.clone(),
            runtime_plugins: crate::config::base_client_runtime_plugins(conf),
        };
        if let Err(err) = Self::validate_config(&handle) {
            panic!("Invalid client configuration: {err}");
        }
        Self {
            handle: ::std::sync::Arc::new(handle),
        }
    }

    /// Returns the client's configuration.
    pub fn config(&self) -> &crate::Config {
        &self.handle.conf
    }

    fn validate_config(handle: &Handle) -> Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
        handle
            .runtime_plugins
            .apply_client_configuration(&mut cfg)?
            .validate_base_client_config(&cfg)?;
        Ok(())
    }
}

impl Client {
    /// Creates a new client from an [SDK Config](::aws_types::sdk_config::SdkConfig).
    ///
    /// # Panics
    ///
    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If
    ///   you experience this panic, set the `sleep_impl` on the Config passed into this function to
    ///   fix it.
    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience
    ///   this panic, set the `http_connector` on the Config passed into this function to fix it.
    /// - This method will panic if no `BehaviorVersion` is provided. If you experience this panic,
    ///   set `behavior_version` on the Config or enable the `behavior-version-latest` Cargo
    ///   feature.
    #[track_caller]
    pub fn new(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }
}

mod allow_vended_log_delivery_for_resource;

mod associate_customization_permission;

mod create_customization;

mod create_profile;

/// Operation customization and supporting types.
pub mod customize;

mod delete_customization;

mod delete_customization_permissions;

mod delete_profile;

mod disassociate_customization_permission;

mod generate_recommendations;

mod get_customization;

mod list_customization_permissions;

mod list_customization_versions;

mod list_customizations;

mod list_profiles;

mod list_tags_for_resource;

mod lock_service_linked_role;

mod tag_resource;

mod unlock_service_linked_role;

mod untag_resource;

mod update_customization;

mod update_profile;
