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
        let _ = SEQUENTIAL_EXEC.lock();
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
    }

    #[test]
    #[should_panic]
    fn without_init() {
        let _ = SEQUENTIAL_EXEC.lock();
        reset();
        #[derive(Debug, Eq, PartialEq)]
        struct A;
        let _a = global_provider::get_dependency::<A>();
    }

}
