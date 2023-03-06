use crate::tag::Tag;
use crate::tag_definitions::TagDefinitions;

#[derive(Default)]
pub struct TagValidator;

impl TagValidator {
    pub fn validate(&self, tag: &Tag, defs: &TagDefinitions) -> bool {
        let def = defs.get(tag.id)
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