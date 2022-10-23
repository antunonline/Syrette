//! When configurator for a binding for types inside of a [`IAsyncDIContainer`].
//!
//! [`IAsyncDIContainer`]: crate::di_container::asynchronous::IAsyncDIContainer
use std::any::type_name;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::di_container::asynchronous::IAsyncDIContainer;
use crate::errors::async_di_container::AsyncBindingWhenConfiguratorError;

/// When configurator for a binding for type 'Interface' inside a [`IAsyncDIContainer`].
///
/// [`IAsyncDIContainer`]: crate::di_container::asynchronous::IAsyncDIContainer
pub struct AsyncBindingWhenConfigurator<Interface, DIContainerType>
where
    Interface: 'static + ?Sized + Send + Sync,
    DIContainerType: IAsyncDIContainer,
{
    di_container: Arc<DIContainerType>,
    interface_phantom: PhantomData<Interface>,
}

impl<Interface, DIContainerType> AsyncBindingWhenConfigurator<Interface, DIContainerType>
where
    Interface: 'static + ?Sized + Send + Sync,
    DIContainerType: IAsyncDIContainer,
{
    pub(crate) fn new(di_container: Arc<DIContainerType>) -> Self
    {
        Self {
            di_container,
            interface_phantom: PhantomData,
        }
    }

    /// Configures the binding to have a name.
    ///
    /// # Errors
    /// Will return Err if no binding for the interface already exists.
    pub async fn when_named(
        &self,
        name: &'static str,
    ) -> Result<(), AsyncBindingWhenConfiguratorError>
    {
        let binding = self
            .di_container
            .remove_binding::<Interface>(None)
            .await
            .map_or_else(
                || {
                    Err(AsyncBindingWhenConfiguratorError::BindingNotFound(
                        type_name::<Interface>(),
                    ))
                },
                Ok,
            )?;

        self.di_container
            .set_binding::<Interface>(Some(name), binding)
            .await;

        Ok(())
    }
}
