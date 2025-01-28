use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: u32,
    pub description: String,
    pub amount: f32,
    pub date: String,
    pub category: String,
}
