use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FriendNudge {
    pub user_id: i64,
    pub is_self_send: bool,
    pub is_self_receiver: bool,
}