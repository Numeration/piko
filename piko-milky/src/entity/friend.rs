use fast_str::FastStr;
use serde::{Deserialize, Serialize};
use crate::entity::friend_category::FriendCategory;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Friend {
    pub user_id: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub qid: Option<FastStr>,
    pub nickname: FastStr,
    pub remark: FastStr,
    #[serde(skip_serializing_if="Option::is_none")]
    pub category: Option<FriendCategory>,
}