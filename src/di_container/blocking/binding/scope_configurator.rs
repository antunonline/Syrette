//! Scope configurator for a binding for types inside of a [`DIContainer`].

use std::any::type_name;
use std::marker::PhantomData;

use crate::di_container::blocking::binding::when_configurator::BindingWhenConfigurator;
use crate::di_container::BindingOptions;
use crate::errors::di_container::{BindingScopeConfiguratorError, DIContainerError};
use crate::errors::injectable::InjectableError;
use crate::interfaces::injectable::Injectable;
use crate::provider::blocking::{SingletonProvider, TransientTypeProvider};
use crate::ptr::SingletonPtr;
use crate::util::use_double;

use_double!(crate::dependency_history::DependencyHistory);
use_double!(crate::di_container::blocking::DIContainer);

/// Scope configurator for a binding for type `Interface` inside a [`DIContainer`].
pub struct BindingScopeConfigurator<'di_container, Interface, Implementation>
where
    Interface: 'static + ?Sized,
    Implementation: Injectable<DIContainer>,
{
    di_container: &'di_container mut DIContainer,
    dependency_history_factory: fn() -> DependencyHistory,

    interface_phantom: PhantomData<Interface>,
    implementation_phantom: PhantomData<Implementation>,
}

impl<'di_container, Interface, Implementation>
    BindingScopeConfigurator<'di_container, Interface, Implementation>
where
    Interface: 'static + ?Sized,
    Implementation: Injectable<DIContainer>,
{
    pub(crate) fn new(
        di_container: &'di_container mut DIContainer,
        dependency_history_factory: fn() -> DependencyHistory,
    ) -> Self
    {
        Self {
            di_container,
            dependency_history_factory,
            interface_phantom: PhantomData,
            implementation_phantom: PhantomData,
        }
    }

    /// Configures the binding to be in a transient scope.
    ///
    /// This is the default.
    ///
    /// # Examples
    /// ```
    /// # use syrette::{DIContainer, injectable};
    /// #
    /// # struct Authenticator {}
    /// #
    /// # #[injectable]
    /// # impl Authenticator
    /// # {
    /// #     fn new() -> Self
    /// #     {
    /// #         Self {}
    /// #     }
    /// # }
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut di_container = DIContainer::new();
    ///
    /// di_container
    ///     .bind::<Authenticator>()
    ///     .to::<Authenticator>()?
    ///     .in_transient_scope();
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn in_transient_scope(
        mut self,
    ) -> BindingWhenConfigurator<'di_container, Interface>
    {
        self.set_in_transient_scope();

        BindingWhenConfigurator::new(self.di_container)
    }

    /// Configures the binding to be in a singleton scope.
    ///
    /// # Errors
    /// Will return Err if resolving the implementation fails.
    ///
    /// # Examples
    /// ```
    /// # use std::sync::atomic::{AtomicBool, Ordering};
    /// # use syrette::{DIContainer, injectable};
    /// #
    /// # struct AudioManager
    /// # {
    /// #     is_sound_playing: AtomicBool
    /// # }
    /// #
    /// # #[injectable]
    /// # impl AudioManager
    /// # {
    /// #     fn new() -> Self
    /// #     {
    /// #         Self { is_sound_playing: AtomicBool::new(false) }
    /// #     }
    /// #
    /// #     fn play_long_sound(&self)
    /// #     {
    /// #         self.is_sound_playing.store(true, Ordering::Relaxed);
    /// #     }
    /// #
    /// #     fn is_sound_playing(&self) -> bool
    /// #     {
    /// #        self.is_sound_playing.load(Ordering::Relaxed)
    /// #     }
    /// #
    /// # }
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut di_container = DIContainer::new();
    ///
    /// di_container
    ///     .bind::<AudioManager>()
    ///     .to::<AudioManager>()?
    ///     .in_singleton_scope();
    ///
    /// {
    ///     let audio_manager = di_container.get::<AudioManager>()?.singleton()?;
    ///
    ///     audio_manager.play_long_sound();
    /// }
    ///
    /// let audio_manager = di_container.get::<AudioManager>()?.singleton()?;
    ///
    /// assert!(audio_manager.is_sound_playing());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn in_singleton_scope(
        self,
    ) -> Result<
        BindingWhenConfigurator<'di_container, Interface>,
        BindingScopeConfiguratorError,
    >
    {
        let singleton: SingletonPtr<Implementation> = SingletonPtr::from(
            Implementation::resolve(
                self.di_container,
                (self.dependency_history_factory)(),
            )
            .map_err(BindingScopeConfiguratorError::SingletonResolveFailed)?,
        );

        self.di_container.set_binding::<Interface>(
            BindingOptions::new(),
            Box::new(SingletonProvider::new(singleton)),
        );

        Ok(BindingWhenConfigurator::new(self.di_container))
    }


    /// Configures the binding to be in a singleton scope.
    ///
    /// # Errors
    /// Will return Err if resolving the implementation fails.
    ///
    /// # Examples
    /// ```
    /// # use std::sync::atomic::{AtomicBool, Ordering};
    /// # use syrette::{DIContainer, injectable};
    /// #
    /// # struct AudioManager
    /// # {
    /// #     is_sound_playing: AtomicBool
    /// # }
    /// #
    /// # trait IAudioManager
    /// # {
    /// #     fn is_enabled(&self) -> bool;
    /// # }
    /// #
    /// # #[injectable]
    /// # impl AudioManager
    /// # {
    /// #     fn new() -> Self
    /// #     {
    /// #         Self { is_sound_playing: AtomicBool::new(false) }
    /// #     }
    /// #
    /// #     fn play_long_sound(&self)
    /// #     {
    /// #         self.is_sound_playing.store(true, Ordering::Relaxed);
    /// #     }
    /// #
    /// #     fn is_sound_playing(&self) -> bool
    /// #     {
    /// #        self.is_sound_playing.load(Ordering::Relaxed)
    /// #     }
    /// #
    /// # }
    /// #
    /// # impl IAudioManager for AudioManager {
    /// #  fn is_enabled(&self) -> bool {
    /// #    true
    /// #  }
    /// # }
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut di_container = DIContainer::new();
    ///
    /// di_container
    ///     .bind::<AudioManager>()
    ///     .to::<AudioManager>()?
    ///     .in_singleton_scope();
    ///
    /// di_container
    ///     .bind::<dyn IAudioManager>()
    ///     .to::<AudioManager>()?
    ///     .in_singleton_scope_from_existing();
    ///
    /// {
    ///     let audio_manager = di_container.get::<AudioManager>()?.singleton()?;
    ///     let i_audio_manager = di_container.get::<dyn IAudioManager>()?.singleton()?;
    ///
    ///     audio_manager.play_long_sound();
    /// }
    ///
    /// let audio_manager = di_container.get::<AudioManager>()?.singleton()?;
    ///
    /// assert!(audio_manager.is_sound_playing());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn in_singleton_scope_from_existing(
        self,
    ) -> Result<
        BindingWhenConfigurator<'di_container, Interface>,
        BindingScopeConfiguratorError,
    >
    {
        let singleton: SingletonPtr<Implementation> =
            self.di_container.get::<Implementation>()
                .map_err(|reason| BindingScopeConfiguratorError::SingletonResolveFailed(
                    InjectableError::ResolveFailed {
                        affected: type_name::<Implementation>(),
                        reason: Box::new( reason )

                    }
                ))?
                .singleton()
                .map_err(|reason| BindingScopeConfiguratorError::SingletonResolveFailed(
                    InjectableError::ResolveFailed {
                        affected: type_name::<Implementation>(),
                        reason: Box::new( DIContainerError::SingletonNotFound (reason, type_name::<Implementation>()))
                    }
                ))?;

        self.di_container.set_binding::<Interface>(
            BindingOptions::new(),
            Box::new(SingletonProvider::new(singleton)),
        );

        Ok(BindingWhenConfigurator::new(self.di_container))
    }

    pub(crate) fn set_in_transient_scope(&mut self)
    {
        self.di_container.set_binding::<Interface>(
            BindingOptions::new(),
            Box::new(TransientTypeProvider::<Implementation, DIContainer>::new()),
        );
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::dependency_history::MockDependencyHistory;
    use crate::di_container::blocking::MockDIContainer;
    use crate::test_utils::subjects;

    #[test]
    fn in_transient_scope_works()
    {
        let mut di_container_mock = MockDIContainer::new();

        di_container_mock
            .expect_set_binding::<dyn subjects::IUserManager>()
            .withf(|options, _provider| options.name.is_none())
            .return_once(|_name, _provider| ())
            .once();

        let binding_scope_configurator = BindingScopeConfigurator::<
            dyn subjects::IUserManager,
            subjects::UserManager,
        >::new(
            &mut di_container_mock,
            MockDependencyHistory::new,
        );

        binding_scope_configurator.in_transient_scope();
    }

    #[test]
    fn in_singleton_scope_works()
    {
        let mut di_container_mock = MockDIContainer::new();

        di_container_mock
            .expect_set_binding::<dyn subjects::IUserManager>()
            .withf(|options, _provider| options.name.is_none())
            .return_once(|_name, _provider| ())
            .once();

        let binding_scope_configurator = BindingScopeConfigurator::<
            dyn subjects::IUserManager,
            subjects::UserManager,
        >::new(
            &mut di_container_mock,
            MockDependencyHistory::new,
        );

        assert!(binding_scope_configurator.in_singleton_scope().is_ok());
    }
}
