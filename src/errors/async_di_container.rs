//! Error types for [`AsyncDIContainer`] and it's related structs.
//!
//! [`AsyncDIContainer`]: crate::di_container::asynchronous::AsyncDIContainer

use crate::errors::injectable::InjectableError;
use crate::errors::ptr::SomePtrError;

/// Error type for [`AsyncDIContainer`].
///
/// [`AsyncDIContainer`]: crate::di_container::asynchronous::AsyncDIContainer
#[derive(thiserror::Error, Debug)]
pub enum AsyncDIContainerError
{
    /// Unable to cast a binding for a interface.
    #[error(
        "Unable to cast binding for interface '{interface} with kind '{binding_kind}'"
    )]
    CastFailed
    {
        /// The interface.
        interface: &'static str,

        /// The kind of the found binding.
        binding_kind: &'static str,
    },

    /// Failed to resolve a binding for a interface.
    #[error("Failed to resolve binding for interface '{interface}'")]
    BindingResolveFailed
    {
        /// The reason for the problem.
        #[source]
        reason: InjectableError,

        /// The affected bound interface.
        interface: &'static str,
    },

    /// No binding exists for a interface (and optionally a name).
    #[error(
        "No binding exists for interface '{interface}' {}",
        .name.map_or_else(String::new, |name| format!("with name '{name}'"))
    )]
    BindingNotFound
    {
        /// The interface that doesn't have a binding.
        interface: &'static str,

        /// The name of the binding if one exists.
        name: Option<&'static str>,
    },

    /// A interface has not been marked async.
    #[error("Interface '{0}' has not been marked async")]
    InterfaceNotAsync(&'static str),

    /// A pointer mismatch
    #[error("Singleton for type '{1}' not found. Please ensure that such type is registered as ThreadSafeSingletonPtr.")]
    SingletonPtrNotFound(SomePtrError, &'static str)
}

/// Error type for [`AsyncBindingBuilder`].
///
/// [`AsyncBindingBuilder`]: crate::di_container::asynchronous::binding::builder::AsyncBindingBuilder
#[derive(thiserror::Error, Debug)]
pub enum AsyncBindingBuilderError
{
    /// A binding already exists for a interface.
    #[error("Binding already exists for interface '{0}'")]
    BindingAlreadyExists(&'static str),
}

/// Error type for [`AsyncBindingScopeConfigurator`].
///
/// [`AsyncBindingScopeConfigurator`]: crate::di_container::asynchronous::binding::scope_configurator::AsyncBindingScopeConfigurator
#[derive(thiserror::Error, Debug)]
pub enum AsyncBindingScopeConfiguratorError
{
    /// Resolving a singleton failed.
    #[error("Resolving the given singleton failed")]
    SingletonResolveFailed(#[from] InjectableError),
}

/// Error type for [`AsyncBindingWhenConfigurator`].
///
/// [`AsyncBindingWhenConfigurator`]: crate::di_container::asynchronous::binding::when_configurator::AsyncBindingWhenConfigurator
#[derive(thiserror::Error, Debug)]
pub enum AsyncBindingWhenConfiguratorError
{
    /// A binding for a interface wasn't found.
    #[error("A binding for interface '{0}' wasn't found'")]
    BindingNotFound(&'static str),
}
