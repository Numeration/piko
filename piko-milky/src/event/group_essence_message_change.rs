use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupEssenceMessageChange {
    pub group_id: i64,
    pub message_seq: i64,
    #[serde(default)]
    pub is_set: bool,
}