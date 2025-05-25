

pub mod private_message {
    use serde::{Deserialize, Serialize};
    use crate::impl_request;
    use crate::entity::OutgoingSegment;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Send {
        pub user_id: i64,
        pub message: Vec<OutgoingSegment>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct SendResponse {
        pub message_seq: i64,
        pub time: i64,
        pub client_seq: i64,
    }

    impl_request!{"send_private_message"| Send, SendResponse}

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Recall {
        pub user_id: i64,
        pub message_seq: i64,
        pub client_seq: i64,
    }

    impl_request!{"recall_private_message"| Recall}
}

pub mod group_message {
    use serde::{Deserialize, Serialize};
    use crate::impl_request;
    use crate::entity::OutgoingSegment;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Send {
        pub group_id: i64,
        pub message: Vec<OutgoingSegment>,
    }
    
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct SendResponse {
        pub message_seq: i64,
        pub time: i64,
    }

    impl_request!{"send_group_message"| Send, SendResponse}


    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Recall {
        pub group_id: i64,
        pub message_seq: i64,
    }

    impl_request!{"recall_group_message"| Recall}
}

#[allow(clippy::module_inception)]
pub mod message {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;
    use crate::entity::IncomingMessage;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        pub message_scene: FastStr,
        pub peer_id: i64,
        pub message_seq: i64,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct GetResponse {
        pub message: IncomingMessage,
    }

    impl_request!{"get_message"| Get, GetResponse}
}

pub mod history_messages {
    use serde::{Deserialize, Serialize};
    use crate::common::{Direction, MessageScene};
    use crate::impl_request;
    use crate::entity::IncomingMessage;
    
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        pub message_scene: MessageScene,
        pub peer_id: i64,
        pub start_message_seq: i64,
        pub direction: Direction,
        #[serde(default = "default_limit")]
        pub limit: i32,
    }
    
    fn default_limit() -> i32 {
        20
    }
    
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct GetResponse {
        pub messages: Vec<IncomingMessage>,
    }

    impl_request!{"get_history_messages"| Get, GetResponse}
}

pub mod resource_temp_url {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        pub resource_id: FastStr,
    }
    
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct GetResponse {
        pub url: FastStr,
    }

    impl_request!{"get_resource_temp_url"| Get, GetResponse}
}

pub mod forwarded_messages {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;
    use crate::entity::IncomingMessage;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        pub forward_id: FastStr,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct GetResponse {
        pub messages: Vec<IncomingMessage>,
    }

    impl_request!{"get_forwarded_messages"| Get, GetResponse}
}