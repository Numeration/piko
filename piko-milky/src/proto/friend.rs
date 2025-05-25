


pub mod friend_nudge {
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Send {
        pub user_id: i64,
        #[serde(default)]
        pub is_self: bool,
    }

    impl_request!{"send_friend_nudge"| Send}
}

pub mod profile_like {
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Send {
        pub user_id: i64,
        #[serde(default = "default_count")]
        pub count: i32,
    }
    
    fn default_count() -> i32 {
        1
    }

    impl_request!{"send_profile_like"| Send}
}