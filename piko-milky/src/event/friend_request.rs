use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FriendRequest {
    pub request_id: FastStr,
    pub operator_id: i64,
    pub comment: FastStr,
    pub via: FastStr,
}
