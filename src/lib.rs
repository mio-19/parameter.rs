use thread_local::ThreadLocal;
use std::cell::{RefCell};
use arc_swap::ArcSwap;
use std::sync::Arc;

pub struct Parameter<T: Clone + Send> {
    base: ArcSwap<T>,
    override_value: ThreadLocal<RefCell<Option<T>>>,
}

impl <T: Clone + Send> Parameter<T> {
    pub fn new(x: T) -> Parameter<T> {
        Parameter { base: ArcSwap::from(Arc::new(x)), override_value: ThreadLocal::new() }
    }
    pub fn set(&self, x: T) {
        self.base.store(Arc::new(x));
    }
    pub fn with<U, F>(&self, x: T, f: F) -> U where F: FnOnce() -> U {
        let origin = self.override_value.get_or(|| RefCell::new(None)).replace(Some(x));
        let result = f();
        let cell = self.override_value.get().unwrap();
        cell.replace(origin);
        result
    }
    pub fn get_clone(&self) -> T {
        if let Some(cell) = self.override_value.get() {
            if let Some(override_value) = &*cell.borrow() {
                return override_value.clone();
            }
        }
        (**self.base.load()).clone()
    }
}
