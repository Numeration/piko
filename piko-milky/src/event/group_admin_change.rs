use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupAdminChange {
    pub group_id: i64,
    pub user_id: i64,
    #[serde(default)]
    pub is_set: bool,
}