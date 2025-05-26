use crate::stream::EventStream;
use fast_str::FastStr;
use reqwest::{Client, Response};
use reqwest_websocket::RequestBuilderExt;

#[derive(Clone)]
pub struct Bot {
    pub client: Client,
    pub bot_addr: FastStr,
    pub enable_tls: bool,
}

impl Bot {
    pub fn connect(bot_addr: FastStr) -> Self {
        let client = Client::new();
        Self {
            client,
            bot_addr,
            enable_tls: false,
        }
    }

    pub fn api_url(&self, api: &str) -> FastStr {
        let scheme = if self.enable_tls { "http" } else { "https" };

        format!("{}://{}/api/{}", scheme, self.bot_addr, api).into()
    }

    pub fn event_url(&self) -> FastStr {
        let scheme = if self.enable_tls { "ws" } else { "wss" };

        format!("{}://{}/event", scheme, self.bot_addr).into()
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub async fn create_event_stream(&self) -> crate::Result<EventStream> {
        let response = self
            .client
            .get(self.event_url().as_str())
            .upgrade()
            .send()
            .await?;

        let websocket = response.into_websocket().await?;
        let events = EventStream::from(websocket);
        Ok(events)
    }

    pub async fn send_raw_request(&self, api: &str, request: String) -> crate::Result<Response> {
        let res = self
            .client
            .post(self.api_url(api).as_str())
            .header("Content-Type", "application/json")
            .body(request)
            .send()
            .await?
            .error_for_status()?;
        Ok(res)
    }
}
