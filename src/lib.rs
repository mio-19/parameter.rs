use thread_local::ThreadLocal;
use std::cell::{RefCell};
use arc_swap::ArcSwap;
use std::sync::Arc;
use owning_ref::ArcRef;

pub struct Parameter<T: Send + Sync> {
    base: ArcSwap<T>,
    override_value: ThreadLocal<RefCell<Option<Arc<T>>>>,
}

impl<T: Send + Sync> Parameter<T> {
    pub fn new(x: T) -> Parameter<T> {
        Parameter { base: ArcSwap::from(Arc::new(x)), override_value: ThreadLocal::new() }
    }
    pub fn set(&self, x: T) {
        self.base.store(Arc::new(x));
    }
    pub fn with<U, F>(&self, x: T, f: F) -> U where F: FnOnce() -> U {
        let origin = self.override_value.get_or(|| RefCell::new(None)).replace(Some(Arc::new(x)));
        let result = f();
        self.override_value.get().unwrap().replace(origin);
        result
    }
    fn get_arc(&self) -> Arc<T> {
        if let Some(cell) = self.override_value.get() {
            if let Some(override_value) = &*cell.borrow() {
                return override_value.clone();
            }
        }
        (*self.base.load()).clone()
    }
    pub fn get(&self) -> ArcRef<T> {
        ArcRef::new(self.get_arc())
    }
}
