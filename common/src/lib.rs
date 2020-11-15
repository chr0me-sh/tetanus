pub mod util;

use std::{net, num, fmt};
use std::io::{self, Write, Read};
use std::convert::From;
use std::convert::TryFrom;

macro_rules! cast_err {
    ( $err:ty => $src:ty, $dst:expr) => (
        impl From<$src> for $err {
            fn from(e: $src) -> Self { $dst(e) }
        }
    )
}

#[derive(Debug)]
pub enum Error {
    Argument,
    Io(io::Error),
    ParseAddr(net::AddrParseError),
    ParseInt(num::ParseIntError)
}

cast_err!(Error => io::Error, Error::Io);
cast_err!(Error => net::AddrParseError, Error::ParseAddr);
cast_err!(Error => num::ParseIntError, Error::ParseInt);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::Io(e)        => write!(f, "IO error: {}", e),
            Error::Argument     => write!(f, "Usage: ./agent [ip] [port]"),
            Error::ParseAddr(e) => write!(f, "Address parse error: {}", e),
            Error::ParseInt(e)  => write!(f, "Integer parse error: {}", e),
        }
    }
}

const MSG_HEADER: [u8; 4] = [84, 84, 78, 83];

#[derive(Debug)]
pub struct Message {
    header: [u8; 4],    
    kind:   MessageKind,
}

#[derive(Debug)]
pub enum MessageKind {
    IDENT,
    UNK,
}

impl Message {
    pub fn new(kind: MessageKind) -> Message {
        Message { header: MSG_HEADER, kind }
    }

    pub fn from_bytes(buf: &[u8; 128]) -> Message {
        Message {
            // TODO: Handle this rather than panic
            header: <[u8; 4]>::try_from(&buf[0..4]).unwrap(),
            kind: {
                match buf[4] as u8 {
                    1 => MessageKind::IDENT,
                    _ => MessageKind::UNK,
                }
            }
        }
    }

    pub fn to_bytes(&self) -> [u8; 128] {
        let mut buf: [u8; 128] = [0; 128];

        for (i, n) in self.header.iter().enumerate() { buf[i] = *n };

        buf[4] = match &self.kind {
            MessageKind::IDENT => 1,
            MessageKind::UNK => 9,
        };

        buf
    }

    pub fn identify() -> Message {
        Message::new(MessageKind::IDENT)
    }
}

impl From<MessageKind> for Message {
    fn from(kind: MessageKind) -> Message {
        Message::new(kind)
    }
}

pub struct Agent {
    id:   usize,
    addr: net::IpAddr,
    conn: net::TcpStream
}

impl Agent {
    pub fn new(id: usize, conn: net::TcpStream) -> Agent {
        Agent {
            id,
            addr: conn.peer_addr().unwrap().ip(),
            conn
        }
    }

    pub fn send(&mut self, msg: Message) -> Result<(), Error> {
        self.conn.write(&msg.to_bytes())?;
        Ok(())
    }

    pub fn recv(&mut self) -> Option<io::Result<Message>> {
        let mut buf: [u8; 128] = [0; 128];

        match self.conn.read(&mut buf) {
            Ok(_) => Some(Ok(Message::from_bytes(&buf))),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => None,
            Err(e) => Some(Err(e))
        }
    }
}