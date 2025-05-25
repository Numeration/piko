use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupAnnouncement {
    pub group_id: i64,
    pub announcement_id: FastStr,
    pub user_id: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub time: Option<i64>,
    pub content: FastStr,
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<FastStr>,
}