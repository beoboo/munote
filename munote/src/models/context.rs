use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::rc::Rc;
use crate::duration::Duration;

pub struct Ptr<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Ptr{ inner: self.inner.clone() }
    }
}

impl<T> Ptr<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(inner))
        }
    }

    pub fn borrow<'a>(&'a self) -> Ref<'a, T> where T:'a {
        (*self.inner).borrow()
    }

    pub fn borrow_mut<'a>(&'a mut self) -> RefMut<'a, T> where T:'a {
        (*self.inner).borrow_mut()
    }
}

impl <T: Default> Default for Ptr<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

pub struct Context {
    pub octave: i8,
    pub duration: Duration,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            octave: 1,
            duration: Duration::default(),
        }
    }
}

pub type ContextPtr = Ptr<Context>;