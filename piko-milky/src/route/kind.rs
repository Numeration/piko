

pub trait EventKind: Sync + Send + 'static {
    type EventData: Sync + Send + 'static;
}

macro_rules! def_event_kind {
    ($event_name:ident) => {
        pub struct $event_name;
        
        impl EventKind for $event_name {
            type EventData = $crate::event::$event_name;
        }
    };
}

macro_rules! def_event_kinds {
    ($($event_name:ident),*) => {
        $(def_event_kind!($event_name);)*
    };
}

def_event_kinds! {
    MessageReceive,
    MessageRecall,
    FriendRequest,
    GroupJoinRequest,
    GroupInvitedJoinRequest,
    GroupInvitationRequest,
    FriendNudge,
    FriendFileUpload,
    GroupAdminChange,
    GroupEssenceMessageChange,
    GroupMemberIncrease,
    GroupMemberDecrease,
    GroupNameChange,
    GroupMessageReaction,
    GroupMute,
    GroupWholeMute,
    GroupNudge,
    GroupFileUpload
}

