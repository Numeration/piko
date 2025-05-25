

#[allow(clippy::module_inception)]
pub mod request {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Accept {
        pub request_id: FastStr,
    }

    impl_request!{"accept_request"| Accept}

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Reject {
        pub request_id: FastStr,
        pub reason: FastStr,
    }
    
    impl_request!{"reject_request"| Reject}
}