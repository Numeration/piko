use crate::bot::Bot;

pub mod system;
pub mod message;
pub mod friend;
pub mod group;
pub mod request;
pub mod file;

pub trait Request {
    type Response;

    #[allow(dead_code)]
    fn send(&self, bot: &Bot) -> impl Future<Output = crate::Result<Self::Response>> + Send;
}

#[macro_export]
macro_rules! impl_request {
    ($api:literal| $method:ident, $response:ident) => {
        impl $crate::proto::Request for $method {
            type Response = $response;

            async fn send(&self, bot: &$crate::Bot) -> $crate::Result<Self::Response> {
                let req = sonic_rs::to_string(self)?;
                let res = bot.send_raw_request($api, req).await?;
                let body = res.bytes().await?;
                let bot_res = sonic_rs::from_slice::<Self::Response>(&body)?;
                Ok(bot_res)
            }
        }
    };

    ($api:literal| $method:ident) => {
        impl $crate::proto::Request for $method {
            type Response = ();

            async fn send(&self, bot: &$crate::Bot) -> $crate::Result<Self::Response> {
                let req = sonic_rs::to_string(self)?;
                bot.send_raw_request($api, req).await?;
                Ok(())
            }
        }
    }
}