//! Interface for structs that can be injected into or be injected to.
use crate::errors::injectable::ResolveError;
use crate::libs::intertrait::CastFrom;
use crate::ptr::TransientPtr;
use crate::DIContainer;

/// Interface for structs that can be injected into or be injected to.
pub trait Injectable: CastFrom
{
    /// Resolves the dependencies of the injectable.
    ///
    /// # Errors
    /// Will return `Err` if resolving the dependencies fails.
    fn resolve(
        di_container: &DIContainer,
        dependency_history: Vec<&'static str>,
    ) -> error_stack::Result<TransientPtr<Self>, ResolveError>
    where
        Self: Sized;
}
