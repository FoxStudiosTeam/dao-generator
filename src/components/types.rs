use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct Type {
    #[serde(rename = "rustType")]
    pub rust_type: String,
}