use std::marker::PhantomData;
use typemap::{Key, TypeMap};

struct ProviderFunction<T>(Box<dyn Fn() -> T>);

impl<T> ProviderFunction<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> T + 'static
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

pub struct DependencyProvider {
    providers: TypeMap,
}

impl DependencyProvider {
    pub fn new() -> Self {
        DependencyProvider {
            providers: TypeMap::new(),
        }
    }
    pub fn set<T, F>(mut self, f: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        self.providers
            .insert::<Depenency<T>>(ProviderFunction::new(f));
        self
    }
    pub fn get<T>(&self) -> Option<T>
    where
        T: 'static,
    {
        self.providers.get::<Depenency<T>>().map(|f| f.call())
    }
}

#[cfg(test)]
mod tests {
    use super::DependencyProvider;

    #[derive(Debug, Eq, PartialEq)]
    struct A;
    #[derive(Debug, Eq, PartialEq)]
    struct B(i32);
    #[derive(Debug, Eq, PartialEq)]
    struct C;

    #[test]
    fn it_works() {
        let d = DependencyProvider::new().set(|| A).set(|| B(0));
        let a = d.get::<A>();
        assert_eq!(Some(A), a);
        let b = d.get::<B>();
        assert_eq!(Some(B(0)), b);
        let c = d.get::<C>();
        assert_eq!(None, c);
        let d = d.set(|| B(42));
        let b = d.get::<B>();
        assert_eq!(Some(B(42)), b);
    }
}
