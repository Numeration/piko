use fast_str::FastStr;
use serde::{Deserialize, Serialize};
use crate::entity::outgoing_segment::OutgoingSegment;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct OutgoingForwardedMessage {
    pub user_id: i64,
    pub name: FastStr,
    pub segments: Vec<OutgoingSegment>,
}
