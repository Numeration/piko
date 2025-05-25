use fast_str::FastStr;
use serde::{Deserialize, Serialize};
use crate::entity::incoming_segment::IncomingSegment;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct IncomingForwardedMessage {
    pub user_id: i64,
    pub name: FastStr,
    pub segments: Vec<IncomingSegment>,
}