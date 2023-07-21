use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct FlatRecord {
    pub seqname: String,
    pub source: String,
    pub feature: String,
    pub start: u64,
    pub end: u64,
    pub score: Option<String>,
    pub strand: Option<String>,
    pub frame: Option<String>,
    #[serde(flatten)]
    pub attributes: Map<String, Value>,
}
impl FlatRecord {
    pub fn has_attribute(&self, variable: &str) -> bool {
        self.attributes.contains_key(variable)
    }
}
