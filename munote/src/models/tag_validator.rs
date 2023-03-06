use anyhow::bail;
use anyhow::Result;

use crate::tag::Tag;
use crate::tag_definitions::TagDefinitions;

#[derive(Default)]
pub struct TagValidator;

impl TagValidator {
    pub fn validate(&self, tag: &Tag, defs: &TagDefinitions) -> Result<()> {
        // println!("\n\nValidating {tag:?}");
        let def = defs.get(tag.id)
            .expect(&format!("Undefined tag ID: {:?}", tag.id));

        // println!("\n{def:?}");

        if tag.ty != def.ty {
            bail!("Invalid tag type (expected: {:?}, found: {:?})", def.ty, tag.ty)
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tag::TagType;
    use crate::tag_id::TagId;

    use super::*;

    fn validate(tag: &Tag) -> Result<()> {
        let defs = TagDefinitions::default();
        let validator = TagValidator::default();

        validator.validate(&tag, &defs)
    }

    #[test]
    fn valid_tag() -> Result<()> {
        let tag = Tag::from_id(TagId::Accelerando).with_type(TagType::Range);

        assert!(validate(&tag).is_ok());

        Ok(())
    }
    //
    // #[test]
    // fn invalid_type() {
    //     let tag = Tag::from_id(TagId::Accelerando);
    //
    //     assert!(validate(&tag));
    // }
}