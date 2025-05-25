use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupWholeMute {
    pub group_id: i64,
    pub operator_id: i64,
    #[serde(default)]
    pub is_mute: bool,
}
