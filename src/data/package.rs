use serde::{Deserialize, Serialize};
use serde_json::Value as Value;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub android_file_name: String,
    pub pc_file_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    pub descriptor: Descriptor,
    pub config: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Descriptor {
    pub object_name: String,
    pub author: String,
    pub description: String
}