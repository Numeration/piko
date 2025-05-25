use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupNudge {
    pub group_id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
}