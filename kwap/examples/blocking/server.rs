use std::thread::JoinHandle;

use kwap::blocking::server::{Action, Continue};
use kwap::net::Addrd;
use kwap::platform::Std;
use kwap::req::Req;
use kwap::resp::{code, Resp};

fn exit_respond(req: &Addrd<Req<Std>>) -> (Continue, Action<Std>) {
  let resp = req.as_ref().map(|req| Resp::for_request(req.clone())).map(|mut resp| {
                                                                     resp.set_code(code::CONTENT);
                                                                     resp.set_payload("goodbye, world!".bytes());
                                                                     resp
                                                                   });
  match req.data().path().unwrap() {
    | Some("exit") => (Continue::Yes, Action::Send(resp.map(Into::into))),
    | _ => (Continue::Yes, Action::Nop),
  }
}

fn exit(req: &Addrd<Req<Std>>) -> (Continue, Action<Std>) {
  match req.data().path().unwrap() {
    | Some("exit") => (Continue::No, Action::Exit),
    | _ => (Continue::Yes, Action::Nop),
  }
}

fn say_hello(req: &Addrd<Req<Std>>) -> (Continue, Action<Std>) {
  match req.data().path().unwrap() {
    | Some("hello") => {
      let resp = req.as_ref().map(|req| Resp::for_request(req.clone())).map(|mut resp| {
                                                                         resp.set_code(code::CONTENT);
                                                                         resp.set_payload("hello, world!".bytes());
                                                                         resp
                                                                       });
      (Continue::No, Action::Send(resp.map(Into::into)))
    },
    | _ => (Continue::Yes, Action::Nop),
  }
}

fn not_found(req: &Addrd<Req<Std>>) -> (Continue, Action<Std>) {
  let resp = req.as_ref().map(|req| Resp::for_request(req.clone())).map(|mut resp| {
                                                                     resp.set_code(code::NOT_FOUND);
                                                                     resp
                                                                   });
  (Continue::No, Action::Send(resp.map(Into::into)))
}

pub fn spawn() -> JoinHandle<()> {
  std::thread::Builder::new().stack_size(32 * 1024 * 1024)
                             .spawn(|| {
                               let mut server = kwap::blocking::Server::try_new([127, 0, 0, 1], 5683).unwrap();
                               server.middleware(&exit_respond);
                               server.middleware(&exit);
                               server.middleware(&say_hello);
                               server.middleware(&not_found);
                               let out = server.start();

                               if out.is_err() {
                                 eprintln!("server panicked! {:?}", out);
                               }
                             })
                             .unwrap()
}

#[allow(dead_code)]
fn main() {
  spawn().join().unwrap();
}