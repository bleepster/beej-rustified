use std::net::ToSocketAddrs;
use std::error::Error;

use clap::App as showIp;
use clap::Arg;

fn main() -> Result<(), Box<dyn Error>>{
    let matches = showIp::new("showIp")
        .version("0.1.0")
        .author("bleepster <earl.lapus@gmail.com>")
        .about("show IP addresses for a host given on the command line")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .takes_value(true)
                .help("host name to use"),
        )
        .get_matches();

    if let Some(host) = matches.value_of("host") {
        // Socket Addresses requires a port, so we sneak in a 0 port to make to_socket_addrs happy
        let addresses = (String::from(host) + ":0").to_socket_addrs()?;
        for address in addresses {
            println!("{:?}", address.ip());
        }
    }

    Ok(())
}
