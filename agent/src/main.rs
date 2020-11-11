use std::env;
use std::process;
use std::net;

fn main() {
    if env::args().len() < 2 {
        println!("Please specify controller address and port.");
        process::exit(1);
    } 

    let cnc_ip: net::IpAddr = match env::args().nth(1)
        .expect("Failed to get IP argument.")
        .parse::<net::IpAddr>() {
            Ok(i)  => i,
            Err(_) => {
                println!("Failed to parse IP address.");
                process::exit(1)
            },
    };

    let cnc_port: u16 = match env::args().nth(2)
        .expect("Failed to get port argument")
        .parse::<u16>() {
            Ok(p) => p,
            Err(_) => {
                println!("Failed to parse port");
                process::exit(1)
            },
    };

    println!("{:?}:{:?}", cnc_ip, cnc_port);
}
