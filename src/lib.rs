//! TODO: create level docs

#![deny(missing_docs)]
#![deny(warnings)]

use std::marker::PhantomData;
use typemap::{Key, ShareMap, TypeMap};

struct ProviderFunction<T>(Box<dyn Fn() -> T + Send + Sync>);

impl<T> ProviderFunction<T> {
    fn new<F>(f: F) -> Self
    where
        F: Fn() -> T + 'static + Send + Sync
    {
        ProviderFunction(Box::new(f))
    }
    fn call(&self) -> T {
        (self.0)()
    }
}

struct Depenency<T: 'static>(PhantomData<T>);

impl<T> Key for Depenency<T> where T: 'static {
    type Value = ProviderFunction<T>;
}

/// A provider for dependencies.
/// Provider functions can be registered for each depndency type.
///
/// # Examples
///
/// ```
/// use dependency_provider::DependencyProvider;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct A;
/// #[derive(Debug, Eq, PartialEq)]
/// struct B(i32);
/// #[derive(Debug, Eq, PartialEq)]
/// struct C;
///
/// let d = DependencyProvider::new()
///     .register(|| A)
///     .register(|| B(0));
/// let a = d.get::<A>();
/// assert_eq!(Some(A), a);
/// let b = d.get::<B>();
/// assert_eq!(Some(B(0)), b);
/// let c = d.get::<C>();
/// assert_eq!(None, c);
/// let d = d.register(|| B(42));
/// let b = d.get::<B>();
/// assert_eq!(Some(B(42)), b);
/// ```
pub struct DependencyProvider {
    providers: ShareMap,
}

impl DependencyProvider {

    /// Create a new instance without any registered provider functions
    pub fn new() -> Self {
        DependencyProvider {
            providers: TypeMap::custom(),
        }
    }

    /// Register a new provider function for a dependency.
    /// The return type of the provider function
    /// is the type of the dependency that is being registered.
    ///
    /// Self is consumed and returned in order to chain calls
    /// while creating the DependencyProvider.
    ///
    /// Calling `register` multiple times for the same dependency type
    /// is allowed, and only the currently last registered provider function
    /// is used to provide the dependency.
    pub fn register<T, F>(mut self, f: F) -> Self
    where
        F: Fn() -> T + 'static + Send + Sync,
        T: 'static,
    {
        self.providers
            .insert::<Depenency<T>>(ProviderFunction::new(f));
        self
    }

    /// Register a provider function for a dependency type
    /// that implements `Default`.
    /// The default implementation is used to provide the depenedency.
    ///
    /// Examples:
    /// ```
    /// use dependency_provider::DependencyProvider;
    ///
    /// #[derive(Debug, Eq, PartialEq, Default)]
    /// struct B(i32);
    ///
    /// let d = DependencyProvider::new()
    ///     .register_default::<B>();
    /// let b = d.get::<B>();
    /// assert_eq!(Some(B::default()), b);
    /// ```
    pub fn register_default<T>(mut self) -> Self
    where T: Default + 'static {
        self.providers
            .insert::<Depenency<T>>(ProviderFunction::new(T::default));
        self
    }

    /// Get an instance of a dependency
    /// by calling a previously registered provider function.
    ///
    /// Returns `None` if no provider function has been registered
    /// for this dependency type.
    pub fn get<T>(&self) -> Option<T>
    where
        T: 'static,
    {
        self.providers.get::<Depenency<T>>().map(|f| f.call())
    }
}

impl Default for DependencyProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::DependencyProvider;
    use lazy_static::lazy_static;

    #[test]
    fn lazy_static_call() {
        #[derive(Debug, Eq, PartialEq)]
        struct A;
        #[derive(Debug, Eq, PartialEq)]
        struct B(i32);
        #[derive(Debug, Eq, PartialEq)]
        struct C;

        lazy_static!{
            static ref PROVIDER: DependencyProvider = {
                DependencyProvider::new()
                    .register(|| A)
                    .register(|| B(0))
            };
        }

        let a = PROVIDER.get::<A>();
        assert_eq!(Some(A), a);
        let b = PROVIDER.get::<B>();
        assert_eq!(Some(B(0)), b);
        let c = PROVIDER.get::<C>();
        assert_eq!(None, c);
    }
}
