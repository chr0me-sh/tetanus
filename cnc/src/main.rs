extern crate tetanus;

use tetanus::util;

use std::io::{self, Read, Write};
use std::{net, thread};
use std::process::exit;

fn get_input(prompt: &str) -> Result<String, tetanus::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn start_listener(sock: net::SocketAddr) -> thread::JoinHandle<io::Result<()>> {
    thread::spawn(move || -> io::Result<()> {
        let lstn = net::TcpListener::bind(sock).unwrap();
        for stream in lstn.incoming() {
            match stream {
                // TODO: Agent connection handling
                Ok(s)  => println!("CONNECT: {:?} -> {:?}", s.peer_addr().unwrap(), s.local_addr().unwrap()),
                Err(e) => return Err(e)
            };
        };
        Ok(())
    })
}

fn run() -> Result<(), tetanus::Error> {
    let mut srv: thread::JoinHandle<io::Result<()>>;

    loop {
        let input: String = get_input("> ")?;
        let cmd: Vec<&str> = input.split_whitespace().collect();

        if cmd.len() < 1 { continue; }

        // TODO: Error handling for command parsing
        match cmd[0] {
            "listen" => srv = start_listener(net::SocketAddr::new(cmd[1].parse()?, cmd[2].parse()?)),
            _ => continue,
        };
    };

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