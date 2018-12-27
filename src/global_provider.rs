use crate::DependencyProvider;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref GLOBAL_PROVIDER: RwLock<Option<DependencyProvider>> = RwLock::new(None);
}

#[allow(dead_code)]
pub fn init(provider: DependencyProvider) {
    (*GLOBAL_PROVIDER.write().unwrap()).replace(provider);
}

#[allow(dead_code)]
pub fn get_dependency<T: 'static>() -> Option<T> {
    (*GLOBAL_PROVIDER.read().unwrap())
        .as_ref()
        .expect("tried to use global dependency provider without calling init()")
        .get()
}

#[macro_export]
macro_rules! inject {
    (let $i:ident: $t:ty) => {
        let $i: $t = global_provider::get_dependency::<$t>()
            .expect("No provider function registered for dependency $t");
    };
    (let mut $i:ident: $t:ty) => {
        let mut $i: $t = global_provider::get_dependency::<$t>()
            .expect("No provider function registered for dependency $t");
    };
}

#[cfg(test)]
mod tests {
    use crate::global_provider;
    use crate::DependencyProvider;

    use lazy_static::lazy_static;
    use std::sync::Mutex;
    lazy_static! {
        static ref SEQUENTIAL_EXEC: Mutex<()> = Mutex::new(());
    }

    fn reset() {
        use super::GLOBAL_PROVIDER;
        (*GLOBAL_PROVIDER.write().unwrap()).take();
    }

    #[test]
    fn global_provider() {
        let lock = SEQUENTIAL_EXEC.lock();
        reset();
        #[derive(Debug, Eq, PartialEq)]
        struct A;
        #[derive(Debug, Eq, PartialEq)]
        struct B(i32);
        #[derive(Debug, Eq, PartialEq)]
        struct C;

        global_provider::init(DependencyProvider::new().register(|| A).register(|| B(0)));

        let a = global_provider::get_dependency::<A>();
        assert_eq!(Some(A), a);
        let b = global_provider::get_dependency::<B>();
        assert_eq!(Some(B(0)), b);
        let c = global_provider::get_dependency::<C>();
        assert_eq!(None, c);
        drop(lock);
    }

    #[test]
    #[should_panic]
    fn without_init() {
        let lock = SEQUENTIAL_EXEC.lock();
        reset();
        #[derive(Debug, Eq, PartialEq)]
        struct A;
        let _a = global_provider::get_dependency::<A>();
        drop(lock);
    }

    #[test]
    fn without_register() {
        let lock = SEQUENTIAL_EXEC.lock();
        reset();
        #[derive(Debug, Eq, PartialEq)]
        struct A;
        global_provider::init(DependencyProvider::new());
        let a = global_provider::get_dependency::<A>();
        assert_eq!(None, a);
        drop(lock);
    }

    #[test]
    fn inject() {
        let lock = SEQUENTIAL_EXEC.lock();
        reset();
        #[derive(Debug, Eq, PartialEq)]
        struct A;
        global_provider::init(DependencyProvider::new().register(|| A));
        inject!(let a: A);
        assert_eq!(A, a);
        drop(lock);
    }

    #[test]
    fn inject_mut() {
        let lock = SEQUENTIAL_EXEC.lock();
        reset();
        #[derive(Debug, Eq, PartialEq)]
        struct A;
        global_provider::init(DependencyProvider::new().register(|| A));
        inject!(let mut a: A);
        assert_eq!(A, a);
        a = A; // must be mut
        assert_eq!(A, a); // must be read
        drop(lock);
    }

    #[test]
    #[should_panic]
    fn inject_without_init() {
        let lock = SEQUENTIAL_EXEC.lock();
        reset();
        #[derive(Debug, Eq, PartialEq)]
        struct A;
        inject!(let _a: A);
        drop(lock);
    }

    #[test]
    #[should_panic]
    fn inject_without_register() {
        let lock = SEQUENTIAL_EXEC.lock();
        reset();
        #[derive(Debug, Eq, PartialEq)]
        struct A;
        global_provider::init(DependencyProvider::new());
        inject!(let _a: A);
        drop(lock);
    }

}
