use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FriendFileUpload {
    pub user_id: i64,
    pub file_id: FastStr,
    pub file_name: FastStr,
    pub file_size: i64,
    pub is_self: bool,
}