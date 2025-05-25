use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FriendCategory {
    pub category_id: i32,
    pub category_name: FastStr,
}