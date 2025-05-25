
pub mod login_info {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get;
    
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct GetResponse {
        pub uin: i64,
        pub nickname: FastStr,
    }
    
    impl_request!{"get_login_info"| Get, GetResponse}
}

pub mod friend_list {
    use crate::entity::Friend;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        #[serde(default)]
        pub no_cache: bool
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct GetResponse {
        pub friends: Vec<Friend>
    }

    impl_request!{"get_friend_list"| Get, GetResponse}
}

pub mod friend_info {
    use crate::entity::Friend;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        pub user_id: i64,
        #[serde(default)]
        pub no_cache: bool,
    }
    
    pub type GetResponse = Friend;

    impl_request!{"get_friend_info"| Get, GetResponse}
}

pub mod group_list {
    use crate::entity::Group;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        #[serde(default)]
        pub no_cache: bool,
    }
    
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct GetResponse {
        pub groups: Vec<Group>,
    }

    impl_request!{"get_group_list"| Get, GetResponse}
}

pub mod group_info {
    use crate::entity::Group;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        pub group_id: i64,
        #[serde(default)]
        pub no_cache: bool,
    }

    pub type GetResponse = Group;

    impl_request!{"get_group_info"| Get, GetResponse}
}

pub mod group_member_list {
    use serde::{Deserialize, Serialize};
    use crate::impl_request;
    use crate::entity::GroupMember;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        pub group_id: i64,
        #[serde(default)]
        pub no_cache: bool,
    }
    
    pub type GetResponse = Vec<GroupMember>;

    impl_request!{"get_group_member_list"| Get, GetResponse}
}

