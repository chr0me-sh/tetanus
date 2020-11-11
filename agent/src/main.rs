mod util;

use std::env;
use std::process::exit;
use std::net;

fn notify_exit(msg: &str) {
    println!("{}", msg);
    exit(1)
}

fn main() {
    if env::args().len() < 3 {
        notify_exit("Usage: ./agent [ip] [port]")
    } 

    let cnc_ip: net::IpAddr = match env::args().nth(1) {
        Some(ip_str) => match ip_str.parse() {
                Ok(ip) => ip,
                Err(_) => return notify_exit("Failed to parse IP address")
            }
        None => panic!("Failed to get address argument")
    };

    let cnc_port: u16 = match env::args().nth(2) {
        Some(p_str) => match p_str.parse() {
            Ok(p)  => p,
            Err(_) => return notify_exit("Failed to parse port number")
        }
        None => panic!("Failed to get port argument")
    };

    println!("{:?}:{:?}", cnc_ip, cnc_port);
}
