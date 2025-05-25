use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupNameChange {
    pub group_id: i64,
    pub name: FastStr,
    pub operator_id: i64,
}