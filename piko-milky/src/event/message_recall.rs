use serde::{Deserialize, Serialize};
use crate::common::MessageScene;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MessageRecall {
    pub message_scene: MessageScene,
    pub peer_id: i64,
    pub message_seq: i64,
    pub sender_id: i64,
    pub operator_id: i64,
}

