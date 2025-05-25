use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupFile {
    pub group_id: i64,
    pub file_id: FastStr,
    pub file_name: FastStr,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_folder_id: Option<FastStr>,
    pub file_size: i64,
    pub uploaded_time: i64,
    pub expire_time: i64,
    pub uploader_id: i64,
    pub downloaded_times: i32,
}