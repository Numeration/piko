use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sex {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Admin,
    Member,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GroupMember {
    pub group_id: i64,
    pub user_id: i64,
    pub nickname: FastStr,
    pub card: FastStr,
    pub title: FastStr,
    pub sex: Sex,
    pub level: i32,
    pub role: Role,
    pub join_time: i64,
    pub last_sent_time: i64,
}
