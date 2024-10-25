// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub type BoxFuture<T> = ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = T> + ::std::marker::Send>>;

pub type SendResult<T, E> = ::std::result::Result<
    T,
    ::aws_smithy_runtime_api::client::result::SdkError<E, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>,
>;

pub trait CustomizableSend<T, E>: ::std::marker::Send + ::std::marker::Sync {
    // Takes an owned `self` as the implementation will internally call methods that take `self`.
    // If it took `&self`, that would make this trait object safe, but some implementing types do not
    // derive `Clone`, unable to yield `self` from `&self`.
    fn send(self, config_override: crate::config::Builder) -> BoxFuture<SendResult<T, E>>;
}
