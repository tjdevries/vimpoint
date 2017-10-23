// TODO: Use logging
extern crate neovim_lib;

mod event;
mod handler;

use std::error::Error;
use std::sync::mpsc;

use neovim_lib::session::Session;
use neovim_lib::neovim::Neovim;

use event::Event;
use handler::RpcHandler;

fn main() {
    use std::process;

    match start_program() {
        Ok(_) => {
            process::exit(0)
        },

        Err(msg) => {
            // error!("{}", msg);
            process::exit(1);
        }
    }
}

fn start_program() -> Result<(), Box<Error>> {
    let (sender, receiver) = mpsc::channel();
    let mut session = try!(Session::new_parent());
    session.start_event_loop_handler(RpcHandler(sender));

    let mut nvim = Neovim::new(session);

    // nvim.command("echom \"Succesful connection: vimpoint\"").unwrap();

    start_event_loop(receiver, nvim);

    return Ok(());
}

fn start_event_loop(receiver: mpsc::Receiver<Event>, mut nvim: Neovim) {
    loop {
        match receiver.recv() {
            Event::VimpointScratchPad => {
                let temp_file_name: String = nvim.call_function("tempfile", [])
                    .expect("tempfile() failed");

                let message = format!("echom 'Wow tempfile was {}'",
                                      "echom \"Wow tempfile was");

                nvim.command(message);
            }

            _ => {
                nvim.command("echom 'We got something that we didn't expect'");
            }
    }
}
