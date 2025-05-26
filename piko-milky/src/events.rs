use crate::route::Router;
use crate::stream::EventStream;
use crate::Bot;
use futures_util::StreamExt;
use std::borrow::Borrow;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tracing::error;

pub struct Events {
    close_flag: Arc<AtomicBool>,
    is_closed: tokio::sync::watch::Receiver<bool>,
}

impl Events {
    pub async fn listen<S: Send + Clone + 'static>(
        bot: impl Borrow<Bot>,
        app: Router<S>,
    ) -> crate::Result<Self> {
        let bot = bot.borrow().clone();
        let stream = bot.create_event_stream().await?;
        Ok(Self::new(stream, app, bot))
    }

    pub async fn event_loop<S: Send + Clone + 'static>(
        mut stream: EventStream,
        mut app: Router<S>,
        bot: Bot,
        close_flag: Arc<AtomicBool>,
        is_closed: tokio::sync::watch::Sender<bool>,
    ) {
        while let Some(event) = stream.next().await {
            if close_flag.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            match event {
                Ok(event) => {
                    app.handle(&bot, event).await;
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        }

        is_closed.send(true).unwrap();
    }

    pub fn new<S: Send + Clone + 'static>(stream: EventStream, app: Router<S>, bot: Bot) -> Self {
        let close_flag = Arc::new(AtomicBool::new(false));
        let (tx, rx) = tokio::sync::watch::channel(false);

        tokio::spawn(Self::event_loop(stream, app, bot, close_flag.clone(), tx));
        Self {
            close_flag,
            is_closed: rx,
        }
    }

    pub async fn join(&self) {
        self.is_closed.clone().wait_for(|&x| x).await.ok();
    }

    pub async fn close(mut self) {
        if !self.close_flag.load(std::sync::atomic::Ordering::Relaxed) {
            self.close_flag
                .store(true, std::sync::atomic::Ordering::Relaxed);
            self.is_closed.wait_for(|&x| x).await.ok();
        }
    }
}
