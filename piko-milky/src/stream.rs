use crate::event::Event;
use crate::error;
use futures::Stream;
use pin_project::pin_project;
use reqwest_websocket::{Message, WebSocket};
use std::pin::Pin;
use std::task::{ready, Context, Poll};

#[pin_project]
pub struct EventStream {
    #[pin]
    raw_event_stream: WebSocket,
}

impl From<WebSocket> for EventStream {
    fn from(raw_event_stream: WebSocket) -> Self {
        Self { raw_event_stream }
    }
}

impl Stream for EventStream {
    
    type Item = Result<Event, crate::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.as_mut().project();
        let mut raw_eve_stream = this.raw_event_stream;
        
        loop {
            match ready!(raw_eve_stream.as_mut().poll_next(cx)) {
                Some(Ok(frame)) => {
                    match frame {
                        Message::Text(event_payload) => {
                            return match sonic_rs::from_str(&event_payload) {
                                Ok(eve) => Poll::Ready(Some(Ok(eve))),
                                Err(err) => {
                                    let proto_err = error::throw_event_json_error(err);
                                    Poll::Ready(Some(Err(proto_err)))
                                }
                            }
                        }
                        Message::Binary(_) => {
                            panic!("Unexpected binary message");
                        }
                        Message::Ping(_) => {}
                        Message::Pong(_) => {}
                        Message::Close { .. } => {
                            return Poll::Ready(None)
                        }
                    }
                },
                Some(Err(err)) => {
                    let io_err = error::throw_event_io_error(err);
                    return Poll::Ready(Some(Err(io_err)));
                },
                None => return Poll::Ready(None),
            }
        }
    }
}