
use std::sync::mpsc;

mod event;
use event::Event;

pub struct RpcHandler(pub mspc::Sender<Event>);
