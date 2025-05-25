use crate::Bot;

pub struct Meta<S> {
    pub bot: Bot,
    pub state: S,
    
    pub time: i64,
    pub self_id: i64,
}