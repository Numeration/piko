use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Group {
    pub group_id: i64,
    pub name: FastStr,
    pub member_count: i32,
    pub max_member_count: i32,
}