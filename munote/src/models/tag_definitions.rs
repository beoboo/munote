use std::collections::HashMap;

use anyhow::Result;
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::tag::TagType;
use crate::tag_id::TagId;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TagParamType {
    Boolean,
    Float,
    Integer,
    String,
    StringOrInt,
    Unit,
}

#[derive(Deserialize)]
pub struct TagParamDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: TagParamType,
    // default: TagParamDefaultValue,
    pub optional: bool,
}

#[derive(Deserialize)]
pub struct TagDefinition {
    #[serde(rename = "type")]
    pub ty: TagType,
    #[serde(default = "Vec::new")]
    pub alternatives: Vec<String>,
    #[serde(default = "Vec::new")]
    pub params: Vec<TagParamDefinition>,
}

// #[derive(Deserialize)]
pub struct TagDefinitions {
    defs: &'static HashMap<TagId, TagDefinition>,
}

impl Default for TagDefinitions {
    fn default() -> Self {
        lazy_static! {
            static ref TAG_DEFS: HashMap<TagId, TagDefinition> = load_defs(include_str!("../../assets/tag_defs.yaml"))
                .expect("Could not load definitions");
        }

        Self {
            defs: &TAG_DEFS
        }
    }
}

impl TagDefinitions {
    pub fn get(&self, id: TagId) -> Option<&TagDefinition> {
        self.defs.get(&id)
    }
}

fn load_defs(input: &str) -> Result<HashMap<TagId, TagDefinition>> {
    println!("Loading: {input}");
    let defs = serde_yaml::from_str(input)?;
    // let data = YamlLoader::load_from_str(data)?;
    // let data = data[0]["defs"].as_vec().unwrap();
    // println!("{:?}", data);
    // for item in data {
    //     if let Yaml::Hash(h) = item {
    //         for (name, v) in h {
    //             println!("\n{name:?}");
    //
    //             for ()
    //             println!("\n{v:?}");
    //         }
    //     }
    //     // let name = h.as_str();
    //     // println!("{name:?}");
    // }

    // let defs = HashMap::default();;

    Ok(defs)
}
