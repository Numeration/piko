use serde::{Deserialize, Serialize};
use crate::entity::friend::Friend;
use crate::entity::group::Group;
use crate::entity::group_member::GroupMember;
use crate::entity::incoming_segment::IncomingSegment;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "message_scene", rename_all = "snake_case")]
pub enum IncomingMessage {
    Friend {
        peer_id: i64,
        message_seq: i64,
        sender_id: i64,
        time: i64,
        segments: Vec<IncomingSegment>,
        friend: Friend,
        client_seq: i64,
    },
    Group {
        peer_id: i64,
        message_seq: i64,
        sender_id: i64,
        time: i64,
        segments: Vec<IncomingSegment>,
        group: Group,
        group_member: GroupMember,
    },
    Temp {
        peer_id: i64,
        message_seq: i64,
        sender_id: i64,
        time: i64,
        segments: Vec<IncomingSegment>,
        group: Group,
    }
}