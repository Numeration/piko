use async_trait::async_trait;
use crate::route::kind::EventKind;
use crate::route::meta::Meta;

#[async_trait]
pub trait Handler<S, K>: Send + 'static
where
    S: Clone,
    K: EventKind,
{
    
    async fn handle(
        &mut self,
        meta: Meta<S>, 
        event: K::EventData
    );
}

pub type BoxHandler<S, K> = Box<dyn Handler<S, K> + Send + 'static>;

pub(super) struct FunctionHandler<F> {
    handler: F,
}

#[async_trait]
impl<S, K, F, Fut> Handler<S, K> for FunctionHandler<F>
where
    S: Clone + Send + 'static,
    K: EventKind,
    F: FnMut(Meta<S>, K::EventData) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send
{
    async fn handle(&mut self, meta: Meta<S>, event: K::EventData) {
        (self.handler)(meta, event).await;
    }
}

pub fn handler<S, K, F, Fut>(handler: F) -> BoxHandler<S, K>
where
    S: Clone + Send + 'static,
    K: EventKind,
    F: FnMut(Meta<S>, K::EventData) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send
{
    Box::new(FunctionHandler { handler })
}