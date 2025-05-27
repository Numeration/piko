use fast_str::FastStr;
use serde::{Deserialize, Serialize};
use crate::common::SubType;

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
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Image {
    pub resource_id: FastStr,
    pub temp_url: FastStr,
    #[serde(skip_serializing_if="Option::is_none")]
    pub summary: Option<FastStr>,
    pub sub_type: SubType,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Record {
    pub resource_id: FastStr,
    pub temp_url: FastStr,
    pub duration: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Video {
    pub resource_id: FastStr,
    pub temp_url: FastStr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Forward {
    pub forward_id: FastStr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MarketFace {
    pub url: FastStr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct LightApp {
    pub app_name: FastStr,
    pub json_payload: FastStr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Xml {
    pub service_id: i32,
    pub xml_payload: FastStr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IncomingSegment {
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
        data: Reply,
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
    MarketFace {
        data: MarketFace,
    },
    LightApp {
        data: LightApp,
    },
    Xml {
        data: Xml,
    }
}
