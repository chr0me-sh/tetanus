use std::net; 
use std::collections::HashMap;

pub struct Menu {
    prompt:  String,
    options: HashMap<String, Command>,
}

impl Menu {
    pub fn new<S: Into<String>>(prompt: S, options: HashMap<String, Command>) -> Menu {
        Menu { prompt: prompt.into(), options }
    }

    pub fn empty<S: Into<String>>(prompt: S) -> Menu {
        Menu::new(prompt, HashMap::new())
    }

    pub fn register(&mut self, cmd: Command) -> () {
        self.options.insert(cmd.name.clone(), cmd);
    }

    pub fn search<S: Into<String>>(&self, cmd: S) -> Option<&Command> {
        self.options.get(&cmd.into())
    }
}

pub struct Command {
    pub name:   String,
    usage:  String,
    nargs:  usize,
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

impl ArgParse<net::SocketAddr> for Command {
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