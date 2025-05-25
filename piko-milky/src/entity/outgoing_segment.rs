use fast_str::FastStr;
use serde::{Deserialize, Serialize};
use crate::common::{qres, SubType};
use crate::entity::outgoing_forwarded_message::OutgoingForwardedMessage;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Text {
    pub text: FastStr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Mention {
    pub user_id: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MentionAll;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Face {
    pub face_id: FastStr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Reply {
    pub message_seq: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_seq: Option<i64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Image {
    pub uri: qres::Url,
    #[serde(skip_serializing_if="Option::is_none")]
    pub summary: Option<FastStr>,
    pub sub_type: SubType,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Record {
    pub uri: qres::Url,
    pub duration: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Video {
    pub uri: qres::Url,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Forward {
    pub messages: Vec<OutgoingForwardedMessage>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutgoingSegment {
    Text {
        data: Text,
    },
    Mention {
        data: Mention,
    },
    MentionAll {
        data: MentionAll,
    },
    Face {
        data: Face,
    },
    Reply {
        reply: Reply,
    },
    Image {
        data: Image,
    },
    Record {
        data: Record,
    },
    Video {
        data: Video,
    },
    Forward {
        data: Forward,
    },
}