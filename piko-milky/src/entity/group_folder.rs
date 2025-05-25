use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupFolder {
    pub group_id: i64,
    pub folder_id: FastStr,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_folder_id: Option<FastStr>,
    pub folder_name: FastStr,
    pub created_time: i64,
    pub last_modified_time: i64,
    pub creator_id: i64,
    pub file_count: i32,
}