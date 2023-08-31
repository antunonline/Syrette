//! When configurator for a binding for types inside of a [`IAsyncDIContainer`].
//!
//! [`IAsyncDIContainer`]: crate::di_container::asynchronous::IAsyncDIContainer
use std::any::type_name;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::di_container::asynchronous::IAsyncDIContainer;
use crate::di_container::BindingOptions;
use crate::errors::async_di_container::AsyncBindingWhenConfiguratorError;

/// When configurator for a binding for type `Interface` inside a [`IAsyncDIContainer`].
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
        self,
        name: &'static str,
    ) -> Result<(), AsyncBindingWhenConfiguratorError>
    {
        let binding = self
            .di_container
            .remove_binding::<Interface>(BindingOptions::new())
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
            .set_binding::<Interface>(BindingOptions::new().name(name), binding)
            .await;

        Ok(())
    }
}

#[cfg(test)]
mod tests
{
    use mockall::predicate::eq;

    use super::*;
    use crate::provider::r#async::MockIAsyncProvider;
    use crate::test_utils::{mocks, subjects_async};

    #[tokio::test]
    async fn when_named_works()
    {
        let mut di_container_mock =
            mocks::async_di_container::MockAsyncDIContainer::new();

        di_container_mock
            .expect_remove_binding::<dyn subjects_async::INumber>()
            .with(eq(BindingOptions::new()))
            .return_once(|_name| Some(Box::new(MockIAsyncProvider::new())))
            .once();

        di_container_mock
            .expect_set_binding::<dyn subjects_async::INumber>()
            .withf(|binding_options, _provider| binding_options.name == Some("awesome"))
            .return_once(|_name, _provider| ())
            .once();

        let binding_when_configurator = AsyncBindingWhenConfigurator::<
            dyn subjects_async::INumber,
            mocks::async_di_container::MockAsyncDIContainer,
        >::new(Arc::new(di_container_mock));

        assert!(binding_when_configurator
            .when_named("awesome")
            .await
            .is_ok());
    }
}
