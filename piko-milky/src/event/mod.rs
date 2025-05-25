mod message_receive;
mod message_recall;
mod friend_request;
mod group_join_request;
mod group_invited_join_request;
mod group_invitation_request;
mod friend_nudge;
mod friend_file_upload;
mod group_admin_change;
mod group_essence_message_change;
mod group_member_increase;
mod group_member_decrease;
mod group_name_change;
mod group_message_reaction;
mod group_mute;
mod group_whole_mute;
mod group_nudge;
mod group_file_upload;

use serde::{Deserialize, Serialize};

pub use {
    message_receive::*,
    message_recall::*,
    friend_request::*,
    group_join_request::*,
    group_invited_join_request::*,
    group_invitation_request::*,
    friend_nudge::*,
    friend_file_upload::*,
    group_admin_change::*,
    group_essence_message_change::*,
    group_member_increase::*,
    group_member_decrease::*,
    group_name_change::*,
    group_message_reaction::*,
    group_mute::*,
    group_whole_mute::*,
    group_nudge::*,
    group_file_upload::*,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum Event {
    MessageReceive {
        time: i64,
        self_id: i64,
        data: MessageReceive,
    },
    MessageRecall {
        time: i64,
        self_id: i64,
        data: MessageRecall,
    },
    FriendRequest {
        time: i64,
        self_id: i64,
        data: FriendRequest,
    },
    GroupJoinRequest {
        time: i64,
        self_id: i64,
        data: GroupJoinRequest,
    },
    GroupInvitedJoinRequest {
        time: i64,
        self_id: i64,
        data: GroupInvitedJoinRequest,
    },
    GroupInvitationRequest {
        time: i64,
        self_id: i64,
        data: GroupInvitationRequest,
    },
    FriendNudge {
        time: i64,
        self_id: i64,
        data: FriendNudge,
    },
    FriendFileUpload {
        time: i64,
        self_id: i64,
        data: FriendFileUpload,
    },
    GroupAdminChange {
        time: i64,
        self_id: i64,
        data: GroupAdminChange,
    },
    GroupEssenceMessageChange {
        time: i64,
        self_id: i64,
        data: GroupEssenceMessageChange,
    },
    GroupMemberIncrease {
        time: i64,
        self_id: i64,
        data: GroupMemberIncrease,
    },
    GroupMemberDecrease {
        time: i64,
        self_id: i64,
        data: GroupMemberDecrease,
    },
    GroupNameChange {
        time: i64,
        self_id: i64,
        data: GroupNameChange,
    },
    GroupMessageReaction {
        time: i64,
        self_id: i64,
        data: GroupMessageReaction,
    },
    GroupMute {
        time: i64,
        self_id: i64,
        data: GroupMute,
    },
    GroupWholeMute {
        time: i64,
        self_id: i64,
        data: GroupWholeMute,
    },
    GroupNudge {
        time: i64,
        self_id: i64,
        data: GroupNudge,
    },
    GroupFileUpload {
        time: i64,
        self_id: i64,
        data: GroupFileUpload,
    }
}
