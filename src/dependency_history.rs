//! Dependency history.

use std::any::type_name;
use std::collections::HashSet;
use std::fmt::{Debug, Display};

const BOLD_MODE: &str = "\x1b[1m";
const RESET_BOLD_MODE: &str = "\x1b[22m";

/// Dependency history.
#[derive(Clone, Debug)]
pub struct DependencyHistory
{
    inner: Vec<&'static str>,
}

impl DependencyHistory
{
    #[must_use]
    pub(crate) fn new() -> Self
    {
        Self { inner: vec![] }
    }
}

#[cfg_attr(test, mockall::automock)]
impl DependencyHistory
{
    #[doc(hidden)]
    pub fn push<Dependency: 'static + ?Sized>(&mut self)
    {
        self.inner.push(type_name::<Dependency>());
    }

    #[doc(hidden)]
    #[allow(clippy::must_use_candidate)]
    pub fn contains<Dependency: 'static + ?Sized>(&self) -> bool
    {
        self.inner.contains(&type_name::<Dependency>())
    }
}

impl Display for DependencyHistory
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let mut found_items = HashSet::new();

        let opt_dupe_item = self.inner.iter().find(|item| {
            if found_items.contains(item) {
                return true;
            }

            found_items.insert(*item);

            false
        });

        for (index, item) in self.inner.iter().enumerate() {
            let mut item_is_dupe = false;

            if let Some(dupe_item) = opt_dupe_item {
                if *item == *dupe_item {
                    formatter
                        .write_fmt(format_args!("{BOLD_MODE}{item}{RESET_BOLD_MODE}"))?;

                    item_is_dupe = true;
                }
            }

            if !item_is_dupe {
                formatter.write_str(item)?;
            }

            if index != self.inner.len() - 1 {
                formatter.write_str(" -> ")?;
            }
        }

        if opt_dupe_item.is_some() {
            formatter.write_str(" -> ...")?;
        }

        Ok(())
    }
}

impl Default for DependencyHistory
{
    fn default() -> Self
    {
        Self::new()
    }
}

impl private::Sealed for DependencyHistory {}

pub(crate) mod private
{
    pub trait Sealed {}
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::test_utils::subjects;

    #[test]
    fn can_push()
    {
        let mut dependency_history = DependencyHistory::new();

        dependency_history.push::<dyn subjects::INumber>();

        assert!(dependency_history
            .inner
            .contains(&type_name::<dyn subjects::INumber>()));
    }

    #[test]
    fn contains_works()
    {
        let mut dependency_history = DependencyHistory::new();

        dependency_history
            .inner
            .push(type_name::<dyn subjects::IUserManager>());

        assert!(dependency_history.contains::<dyn subjects::IUserManager>());

        assert!(!dependency_history.contains::<dyn subjects::INumber>());
    }

    #[test]
    fn display_works()
    {
        trait Ninja {}
        trait Katana {}
        trait Blade {}

        let mut dependency_history = DependencyHistory::new();

        dependency_history.inner.push(type_name::<dyn Ninja>());
        dependency_history.inner.push(type_name::<dyn Katana>());
        dependency_history.inner.push(type_name::<dyn Blade>());

        assert_eq!(
            dependency_history.to_string(),
            format!(
                "{} -> {} -> {}",
                type_name::<dyn Ninja>(),
                type_name::<dyn Katana>(),
                type_name::<dyn Blade>()
            )
        );

        dependency_history.inner.push(type_name::<dyn Katana>());

        assert_eq!(
            dependency_history.to_string(),
            format!(
                concat!(
                    "{} -> {bold_mode}{}{reset_bold_mode} -> {} -> ",
                    "{bold_mode}{}{reset_bold_mode} -> ...",
                ),
                type_name::<dyn Ninja>(),
                type_name::<dyn Katana>(),
                type_name::<dyn Blade>(),
                type_name::<dyn Katana>(),
                bold_mode = BOLD_MODE,
                reset_bold_mode = RESET_BOLD_MODE
            )
        );
    }
}
