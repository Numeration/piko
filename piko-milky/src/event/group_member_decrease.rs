use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupMemberDecrease {
    pub group_id: i64,
    pub user_id: i64,

    #[serde(skip_serializing_if="Option::is_none")]
    pub operator_id: Option<i64>,
}