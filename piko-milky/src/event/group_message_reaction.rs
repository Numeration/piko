use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupMessageReaction {
    pub group_id: i64,
    pub user_id: i64,
    pub message_seq: i64,
    pub face_id: FastStr,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_add: Option<bool>,
}