

pub mod group_name {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Set {
        pub group_id: i64,
        pub name: FastStr,
    }

    impl_request!{"set_group_name"| Set}
}

pub mod group_avatar {
    use serde::{Deserialize, Serialize};
    use crate::common::qres;
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Set {
        pub group_id: i64,
        pub image_uri: qres::Url,
    }

    impl_request!{"set_group_avatar"| Set}
}

pub mod group_member_card {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Set {
        pub group_id: i64,
        pub user_id: i64,
        pub card: FastStr,
    }
    
    impl_request!{"set_group_member_card"| Set}
}

pub mod group_member_admin {
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Set {
        pub group_id: i64,
        pub user_id: i64,
        #[serde(default = "default_is_set")]
        pub is_set: bool,
    }

    fn default_is_set() -> bool {
        true
    }
    
    impl_request!{"set_group_member_admin"| Set}
}

pub mod group_member_mute {
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Set {
        pub group_id: i64,
        pub user_id: i64,
        #[serde(default)]
        pub duration: i64,
    }
    
    impl_request!{"set_group_member_mute"| Set}
}

pub mod group_whole_mute {
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Set {
        pub group_id: i64,
        #[serde(default = "default_is_mute")]
        pub is_mute: bool,
    }
    
    fn default_is_mute() -> bool {
        true
    }
    
    impl_request!{"set_group_whole_mute"| Set}
}


pub mod group_member {
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Kick {
        pub group_id: i64,
        pub user_id: i64,
        #[serde(default = "default_reject_add_request")]
        pub reject_add_request: bool,
    }
    
    fn default_reject_add_request() -> bool {
        true
    }
    
    impl_request!{"kick_group_member"| Kick}
}

pub mod group_announcement_list {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::common::qres;
    use crate::impl_request;
    use crate::entity::GroupAnnouncement;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        pub group_id: i64,
    }
    
    pub type GetResponse = Vec<GroupAnnouncement>;

    impl_request!{"get_group_announcement_list"| Get, GetResponse}

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Send {
        pub group_id: i64,
        pub content: FastStr,
        pub image_uri: qres::Url,
    }

    impl_request!{"send_group_announcement_list"| Send}
    
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Delete {
        pub group_id: i64,
        pub announcement: i64,
    }

    impl_request!{"delete_group_announcement_list"| Delete}
}

#[allow(clippy::module_inception)]
pub mod group {
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Quit {
        pub group_id: i64,
    }

    impl_request!{"quit_group"| Quit}
}

pub mod group_message_reaction {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Send {
        message_seq: i64,
        reaction: FastStr,
        
        #[serde(default = "default_is_add")]
        is_add: bool,
    }
    
    fn default_is_add() -> bool {
        true
    }
    
    impl_request!{"send_group_message_reaction"| Send}
}

pub mod group_nudge {
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Send {
        pub group_id: i64,
        pub user_id: i64,
    }
    
    impl_request!{"send_group_nudge"| Send}
}