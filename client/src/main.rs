use std::io::Error;
use std::io::Read;
use std::net::TcpStream;
use std::string::String;

use dns_lookup::{getaddrinfo, AddrInfoHints, SockType};

fn main() -> Result<(), Error> {
    let port = "3490";
    let hints = AddrInfoHints {
        socktype: SockType::Stream.into(),
        ..AddrInfoHints::default()
    };

    let mut connected_stream: Option<TcpStream> = None;

    let addr_infos = getaddrinfo(None, Some(port), Some(hints))?;
    for addr_info in addr_infos {
        if let Some(c) = match TcpStream::connect(addr_info.unwrap().sockaddr) {
            Ok(connected) => Some(connected),
            Err(_) => None,
        } {
            connected_stream = match c.try_clone() {
                Ok(cloned_connection) => Some(cloned_connection),
                Err(_) => None,
            };
            break;
        }
    }

    if connected_stream.is_some() {
        let mut buf = [0; 64];
        connected_stream.unwrap().read(&mut buf)?;

        let s = String::from_utf8_lossy(&buf);
        println!("client received {}", s);
    } else {
        println!("client: failed to connect");
    }

    Ok(())
}
