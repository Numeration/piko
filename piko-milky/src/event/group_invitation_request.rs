use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupInvitationRequest {
    pub request_id: FastStr,
    pub operator_id: i64,
    pub group_id: i64,
}