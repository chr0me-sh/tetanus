mod util;

use std::env;
use std::process::exit;
use std::str::FromStr;
use std::net;

fn notify_exit(msg: &str) {
    println!("{}", msg);
    exit(1)
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
        notify_exit("Usage: ./agent [ip] [port]")
    } 

    let cnc_ip: net::IpAddr = match parse_arg(1) {
        Some(i) => i,
        None    => return notify_exit("Failed to parse address")
    };

    let cnc_port: u16 = match parse_arg(2) {
        Some(p) => p,
        None    => return notify_exit("Failed to parse port")
    };

    println!("{:?}:{:?}", cnc_ip, cnc_port);
}
