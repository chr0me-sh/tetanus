use std::net; 

pub struct Command<T> {
    name:    String,
    usage:   String,
    nargs:   usize,
    handler: fn()
}

impl Command {
    pub fn new<S: Into<String>>(name: S, usage: S, nargs: usize) -> Command {
        Command { name: name.into(), usage: usage.into(), nargs }
    }

    pub fn usage(&self) {
        println!("{}", self.usage);
    }
}

pub trait ArgParse<T> {
    fn parse(&self, cmd: Vec<&str>) -> Option<T>;
}

impl ArgParse<net::SocketAddr> for Command<Listener> {
    // This kind of sucks
    fn parse(&self, cmd: Vec<&str>) -> Option<net::SocketAddr> {
        if cmd[1..].len() != self.nargs { return None }
        if let Some(ip) = cmd[1].parse::<net::IpAddr>().ok() {
            if let Some(port) = cmd[2].parse::<u16>().ok() {
                return Some(net::SocketAddr::new(ip, port))
            }
        }
        None
    }    
}