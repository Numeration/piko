


pub mod private_file {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::common::qres;
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Upload {
        pub user_id: i64,
        pub file_uri: qres::Url,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct UploadResponse {
        pub file_id: FastStr,
    }
    
    impl_request!{"upload_private_file"| Upload, UploadResponse}
}

pub mod group_file {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::common::qres;
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Upload {
        pub group_id: i64,
        pub file_uri: qres::Url,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct UploadResponse {
        pub file_id: FastStr,
    }

    impl_request!{"upload_group_file"| Upload, UploadResponse}
}

pub mod private_file_download_url {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        pub user_id: i64,
        pub file_id: FastStr,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct GetResponse {
        pub download_url: FastStr,
    }
    
    impl_request!{"get_private_file_download_url"| Get, GetResponse}
}

pub mod group_file_download_url {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        pub group_id: i64,
        pub file_id: FastStr,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct GetResponse {
        pub download_url: FastStr,
    }
    
    impl_request!{"get_group_file_download_url"| Get, GetResponse}
}

pub mod group_files {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;
    use crate::entity::{GroupFile, GroupFolder};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Get {
        pub group_id: i64,
        #[serde(skip_serializing_if="Option::is_none")]
        pub parent_folder_id: Option<FastStr>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct GetResponse {
        pub files: Vec<GroupFile>,
        pub folder: Vec<GroupFolder>,
    }

    impl_request!{"get_group_files"| Get, GetResponse}

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Move {
        pub group_id: i64,
        pub file_id: FastStr,
        #[serde(skip_serializing_if="Option::is_none")]
        pub target_folder_id: Option<FastStr>,
    }

    impl_request!{"move_group_files"| Move}
    
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Rename {
        pub group_id: i64,
        pub file_id: FastStr,
        pub new_name: FastStr,
    }
    
    impl_request!{"rename_group_files"| Rename}

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Delete {
        pub group_id: i64,
        pub file_id: FastStr,
    }
    
    impl_request!{"delete_group_files"| Delete}
}

pub mod group_folder {
    use fast_str::FastStr;
    use serde::{Deserialize, Serialize};
    use crate::impl_request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Create {
        pub user_id: i64,
        pub folder_name: FastStr,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct CreateResponse {
        pub folder_name: FastStr,
    }
    
    impl_request!{"create_group_folder"| Create, CreateResponse}

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Rename {
        pub group_id: i64,
        pub folder_id: FastStr,
        pub new_name: FastStr,
    }
    
    impl_request!{"rename_group_folder"| Rename}
    
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Delete {
        pub group_id: i64,
        pub folder_id: FastStr,
    }
    
    impl_request!{"delete_group_folder"| Delete}
}