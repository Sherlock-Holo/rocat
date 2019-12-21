use std::{env, thread};
use std::io::{self, Read, Write};
use std::process;

pub use file::*;
pub use net::*;

mod file;
mod net;

pub trait ReadWrite: Send + Read + Write {}

pub fn run() {
    let mut cmd_args = env::args().skip(1);

    let from = match cmd_args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("miss from");
            process::exit(1)
        }
    };

    let to = match cmd_args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("miss to");
            process::exit(1)
        }
    };

    let from = parse_from(from);
    let to = parse_to(to);

    let _: () = from.zip(to).map(|(from, to)| {
        let mut from = from.unwrap();
        let mut to = to.unwrap();

        thread::spawn(move || {
            if let Err(err) = io::copy(&mut from, &mut to) {
                eprintln!("copy error {}", err);
            };
        })
    }).map(|handle| {
        let _ = handle.join();
    }).collect();
}

fn parse_from(s: String) -> Box<dyn Iterator<Item=Result<Box<dyn ReadWrite>, io::Error>>> {
    if s.to_uppercase().starts_with("TCP:") {
        let listener = match net::TcpListener::new(&s[4..]) {
            Err(err) => {
                eprintln!("listen failed: {}", err);
                process::exit(1)
            }

            Ok(listener) => listener,
        };

        return Box::new(listener);
    }

    Box::new(file::File::new(s, true))
}

fn parse_to(s: String) -> Box<dyn Iterator<Item=Result<Box<dyn ReadWrite>, io::Error>>> {
    if s.to_uppercase().starts_with("TCP:") {
        return Box::new(net::TcpStream::new(String::from(&s[4..])));
    }

    Box::new(file::File::new(s, false))
}