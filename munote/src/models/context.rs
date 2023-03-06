use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use crate::{duration::Duration, tag::Tag, tag_id::TagId};
use crate::range_tag::RangeTag;
use crate::tag_definitions::{TagDefinition, TagDefinitions};
use crate::tag_validator::TagValidator;

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
    pub defs: TagDefinitions,
    pub validator: TagValidator,
    pub octave: i8,
    pub duration: Duration,
    pub range_tags: HashMap<TagId, RangeTag>,
}

impl Default for Context {
    fn default() -> Self {
        let defs = TagDefinitions::default();
        let validator = TagValidator::default();

        Self {
            defs,
            validator,
            octave: 1,
            duration: Duration::default(),
            range_tags: HashMap::new(),
        }
    }
}

impl Context {
    pub fn add_tag(&mut self, tag: RangeTag) {
        self.range_tags.insert(tag.id, tag);
    }

    pub fn get_tag(&self, id: TagId) -> Option<&RangeTag> {
        self.range_tags.get(&id)
    }

    pub fn validate(&self, tag: &Tag) -> bool {
        self.validator.validate(tag, &self.defs)
    }
}

pub type ContextPtr = Ptr<Context>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_tag() {
        let mut ctx = Context::default();

        let tag = RangeTag::from_id(TagId::Bar, 1, 2);
        ctx.add_tag(tag.clone());

        assert_eq!(ctx.get_tag(TagId::Bar).unwrap(), &tag);
    }
}
