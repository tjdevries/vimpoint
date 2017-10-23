// TODO: Use logging

use std::error::Error;
use std::sync::mspc;

extern crate neovim_lib;

use neovim_lib::session::Session;
use neovim_lib::neovim::Neovim;

use handler::RpcHandler;

fn main() {
    use std::process;

    match start_program() {
        OK(_) => process::exit(0),

        Err(msg) => {
            error!("{}", msg);
            process::exit(1);
        }
    }
}

fn start_program() -> Result<(), Box<Error>> {
    let (sender, receiver) = mspc::channel();
    let mut session = try!(Session::new_parent());
    session.start_event_loop_handler(RpcHandler(sender));

    let mut nvim = Neovim::new(session);

    nvim.command("echom \"Succesful connection: vimpoint\"").unwrap();

    start_event_loop(receiver, nvim);

    Ok(());
}
