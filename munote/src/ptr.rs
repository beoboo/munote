use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct Ptr<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Ptr {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Ptr<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    pub fn borrow<'a>(&'a self) -> Ref<'a, T>
        where
            T: 'a,
    {
        (*self.inner).borrow()
    }

    pub fn borrow_mut<'a>(&'a mut self) -> RefMut<'a, T>
        where
            T: 'a,
    {
        (*self.inner).borrow_mut()
    }
}

impl<T: Default> Default for Ptr<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
