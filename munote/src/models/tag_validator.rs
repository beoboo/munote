use crate::tag::{Tag, TagType};
use crate::tag_definitions::TagDefinitions;

#[derive(Default)]
pub struct TagValidator;

impl TagValidator {
    pub fn validate(&self, tag: &Tag, defs: &TagDefinitions) -> bool {
        // println!("\n\nValidating {tag:?}");
        let def = defs.get(tag.id)
            .expect(&format!("Undefined tag ID: {:?}", tag.id));

        // println!("\n{def:?}");

        match tag.ty {
            TagType::Position => matches!(def.ty, TagType::Position | TagType::Any),
            _ => def.ty != TagType::Position,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tag::TagType;
    use crate::tag_id::TagId;

    use super::*;

    fn validate(tag: &Tag) -> bool {
        let defs = TagDefinitions::default();
        let validator = TagValidator::default();

        validator.validate(&tag, &defs)
    }

    #[test]
    fn valid_tag() {
        let tag = Tag::from_id(TagId::Accelerando).with_type(TagType::Range);

        assert!(validate(&tag));
    }
    //
    // #[test]
    // fn invalid_type() {
    //     let tag = Tag::from_id(TagId::Accelerando);
    //
    //     assert!(validate(&tag));
    // }
}