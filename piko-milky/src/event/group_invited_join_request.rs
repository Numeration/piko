use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupInvitedJoinRequest {
    pub request_id: FastStr,
    pub operator_id: i64,
    pub group_id: i64,
    pub invitee_id: i64,
}