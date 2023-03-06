use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use crate::{duration::Duration, tag::Tag, tag_id::TagId};

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

pub struct Context {
    pub octave: i8,
    pub duration: Duration,
    pub tags: HashMap<TagId, Tag>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            octave: 1,
            duration: Duration::default(),
            tags: HashMap::new(),
        }
    }
}

impl Context {
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag.id, tag);
    }

    pub fn get_tag(&self, id: TagId) -> Option<&Tag> {
        self.tags.get(&id)
    }
}

pub type ContextPtr = Ptr<Context>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_tag() {
        let mut ctx = Context::default();

        let tag = Tag::from_id(TagId::Bar);
        ctx.add_tag(tag.clone());

        assert_eq!(ctx.get_tag(TagId::Bar).unwrap(), &tag);
    }
}
