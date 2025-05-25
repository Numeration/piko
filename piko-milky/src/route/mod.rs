use crate::bot::Bot;
use crate::event::Event;
pub mod kind;
mod meta;
mod handle;

pub use {
    meta::*,
    handle::*,
};
use crate::kind::EventKind;

pub trait RouterTrait<S, K>
where
    S: Clone,
    K: EventKind,
{

    fn route(self, kind: K, handler: BoxHandler<S, K>) -> Self;
}

macro_rules! def_router {
    ($($event_name:ident),*) => {
        #[allow(non_snake_case)]
        pub struct Router<S = ()> {
            state: Option<S>,

            $(
                $event_name : Option<BoxHandler<S, $crate::route::kind::$event_name>>
            ),*
        }

        impl<S> Router<S> {
            pub fn new() -> Self {
                Self {
                    state: None,
                    $(
                        $event_name : None
                    ),*
                }
            }
        }

        impl<S> Default for Router<S> {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

def_router! {
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

macro_rules! impl_route_trait {
    ($event_name:ident) => {
        impl<S> RouterTrait<S, $crate::route::kind::$event_name> for Router<S>
        where
            S: Clone,
        {
            fn route(
                mut self,
                _: $crate::route::kind::$event_name,
                handler: BoxHandler<S, $crate::route::kind::$event_name>
            ) -> Self {
                self.$event_name = Some(handler);
                self
            }
        }
    }
}

macro_rules! impl_route_trait_list {
    ($($event_name:ident),*) => {
        $(impl_route_trait!($event_name);)*
    }
}

impl_route_trait_list! {
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

macro_rules! dispatch_events {
    ($this_var:ident, $bot_var:ident, $state_var:ident, $event_var:ident| $($event_name:ident),*) => {
        match $event_var {
            $(Event::$event_name { time, self_id, data} => {
                if let Some(h) = $this_var.$event_name.as_mut() {
                    let bot = $bot_var.clone();
                    let state = $state_var;
                    h.handle(Meta {bot, state, time, self_id}, data).await
                }
            })*
        }
    };
}

impl<S: Clone + 'static> Router<S> {

    pub fn with_state(mut self, state: S) -> Router<S> {
        self.state = Some(state);
        self
    }

    pub(crate) async fn handle(&mut self, bot: &Bot, event: Event) {
        let state = self.state.clone().unwrap();
        dispatch_events!(self, bot, state, event|
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
        );
    }
}