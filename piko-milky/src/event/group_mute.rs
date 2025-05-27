use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupMute {
    pub group_id: i64,
    pub user_id: i64,
    pub operator_id: i64,
    pub duration: i32,
}