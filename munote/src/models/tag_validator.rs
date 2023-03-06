use yaml_rust::YamlLoader;

use crate::tag::Tag;
use crate::tag_definitions::TagDefinitions;
use crate::tag_id::TagId;

struct TagValidator<'a> {
    defs: &'a TagDefinitions,
}

impl<'a> TagValidator<'a> {
    pub fn new(defs: &'a TagDefinitions) -> Self {
        Self {
            defs
        }
    }

    pub fn validate(&self, tag: &Tag) -> bool {
        let def = self.defs.get(tag.id)
            .expect(&format!("Undefined tag ID: {:?}", tag.id));

        tag.ty == def.ty
    }
}

#[cfg(test)]
mod tests {
    use crate::tag::TagType;
    use crate::tag_id::TagId;

    use super::*;

    fn validate(tag: &Tag) -> bool {
        let defs = TagDefinitions::default();
        let validator = TagValidator::new(&defs);
        validator.validate(&tag)
    }

    #[test]
    fn valid_tag() {
        let tag = Tag::from_id(TagId::Accelerando).with_type(TagType::Range);

        assert!(validate(&tag));
    }

    #[test]
    fn invalid_type() {
        let tag = Tag::from_id(TagId::Accelerando);

        assert!(validate(&tag));
    }
}