
use std::result::Result;
use std::sync::mpsc;

use neovim_lib::{Handler, Value};

use event::Event;

pub struct RpcHandler(pub mpsc::Sender<Event>);

impl RpcHandler {
}

impl Handler for RpcHandler {
    fn handle_request(&mut self, name: &str, args: &Vec<Value>) -> Result<Value, Value> {
        match name {
            "vimpoint.scratchpad" => {
                return Ok(Value::from("this is a temp file"));
            }

            _ => {
                return Err(Value::from(format!("unknown request: {}", name)));
            }
        }
    }
}
