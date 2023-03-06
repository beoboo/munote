use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::tag::TagType;
use crate::tag_id::TagId;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TagParamType {
    Boolean,
    Float,
    Integer,
    String,
    StringOrInt,
    Unit,
}

#[derive(Debug, Deserialize)]
pub struct TagParamDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: TagParamType,
    // default: TagParamDefaultValue,
    pub optional: bool,
}

#[derive(Debug, Deserialize)]
pub struct TagDefinition {
    #[serde(rename = "type")]
    pub ty: TagType,
    #[serde(default = "Vec::new")]
    pub alternatives: Vec<String>,
    #[serde(default = "Vec::new")]
    pub params: Vec<TagParamDefinition>,
}

pub struct TagDefinitions {
    defs: &'static HashMap<TagId, TagDefinition>,
    lookup: &'static HashMap<String, TagId>,
}

impl Default for TagDefinitions {
    fn default() -> Self {
        lazy_static! {
            static ref TAG_DEFS: HashMap<TagId, TagDefinition> = load_defs(include_str!("../../assets/tag_defs.yaml"))
                .expect("Could not load definitions");
            static ref TAG_LOOKUP: HashMap<String, TagId> = build_lookup(&TAG_DEFS)
                .expect("Could not load definitions");
        }

        Self {
            defs: &TAG_DEFS,
            lookup: &TAG_LOOKUP,
        }
    }
}

impl TagDefinitions {
    pub fn get(&self, id: TagId) -> Option<&TagDefinition> {
        self.defs.get(&id)
    }

    pub fn lookup(&self, name: &str) -> Result<TagId> {
        if let Some(alt) = self.lookup.get(name) {
            return Ok(*alt);
        }

        TagId::from_str(name)
            .map_err(|_| anyhow!("Tag ID {name} not found"))
    }
}

fn load_defs(input: &str) -> Result<HashMap<TagId, TagDefinition>> {
    let defs = serde_yaml::from_str(input)?;

    Ok(defs)
}

fn build_lookup(defs: &HashMap<TagId, TagDefinition>) -> Result<HashMap<String, TagId>> {
    let mut lookup = HashMap::new();

    for (id, def) in defs {
        for alt in &def.alternatives {
            if lookup.contains_key(alt) {
                bail!("Alternative {alt} already defined");
            }

            lookup.insert(alt.clone(), *id);
        }
    }

    Ok(lookup)
}
