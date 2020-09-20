use std::io::Error;
use std::net::TcpListener;

use dns_lookup::{getaddrinfo, AddrInfoHints, SockType};

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
                Ok((_socket, addr)) => println!("new client: {:?}", addr),
                Err(e) => println!("attempt from client: {:?} failed", e),
            }
        }
    }

    Ok(())
}
