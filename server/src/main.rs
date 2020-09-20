use std::io::prelude::*;
use std::io::Error;
use std::net::TcpListener;
use std::net::Shutdown;

use dns_lookup::{getaddrinfo, AddrInfoHints, SockType};
use nix::unistd::{fork, ForkResult};

fn main() -> Result<(), Error> {
    let port = "3490";
    let hints = AddrInfoHints {
        socktype: SockType::Stream.into(),
        ..AddrInfoHints::default()
    };

    let mut bound_listener: Option<TcpListener> = None;

    let addr_infos = getaddrinfo(None, Some(port), Some(hints))?;
    for addr_info in addr_infos {
        if let Some(l) = match TcpListener::bind(addr_info.unwrap().sockaddr) {
            Ok(bound) => Some(bound),
            Err(_) => None,
        } {
            bound_listener = match l.try_clone() {
                Ok(cloned_listener) => Some(cloned_listener),
                Err(_) => None,
            };
            break;
        }
    }

    if let Some(listener) = match bound_listener {
        Some(l) => Some(l),
        None => None,
    } {
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("connection accepted: {:?}", addr);
                    match fork() {
                        Ok(ForkResult::Child) => {
                            stream.write(b"Hello world!")?;
                            stream.flush()?;
                            stream.shutdown(Shutdown::Both).expect("Parent shutdown failed")
                        },
                        Ok(ForkResult::Parent {child: _, ..}) => {
                            // XXX: If I shut this down the child gets shutdown too
                            //      Parent and Child are sharing a copy?
                            // stream.shutdown(Shutdown::Both).expect("Parent shutdown failed")
                            println!("Parent...")
                        },
                        Err(_) => {
                            println!("fork() failed")
                        },
                    }
                },
                Err(e) => println!("attempt from client: {:?} failed", e),
            }
        }
    }

    Ok(())
}
