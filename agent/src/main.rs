mod util;

use std::{io, net, env};
use std::process::exit;
use std::str::FromStr;

macro_rules! notify_exit {
    () => ( exit(1) );
    ( $( $s:expr )* ) => ( { println!("{}", $($s)*); exit(1) } );
    ( $( $s:expr )*; $c:expr ) => ( { println!("{}", $($s)*); exit($c) } )
}

fn parse_arg<T: FromStr>(n: usize) -> Option<T> {
    match env::args().nth(n) {
        Some(opt) => match opt.parse::<T>() {
            Ok(res) => Some(res),
            Err(_)  => None
        }
        None => None,
    }
}

fn main() {
    if env::args().len() < 3 {
        notify_exit!("Usage: ./agent [ip] [port]")
    } 

    let cnc_ip: net::IpAddr = match parse_arg(1) {
        Some(i) => i,
        None    => notify_exit!("Failed to parse address")
    };

    let cnc_port: u16 = match parse_arg(2) {
        Some(p) => p,
        None    => notify_exit!("Failed to parse port")
    };

    if let Ok(cnc) = net::TcpStream::connect((cnc_ip, cnc_port)) {

    } else {
        notify_exit!("Failed to connect to remote server");
    }
}
