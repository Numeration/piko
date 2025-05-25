use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupJoinRequest {
    pub request_id: FastStr,
    pub operator_id: i64,
    pub group_id: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment: Option<FastStr>,
}