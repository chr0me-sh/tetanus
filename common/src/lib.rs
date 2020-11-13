use std::net;
use std::io;
use std::num;
use std::fmt;
use std::convert::From;

macro_rules! cast_err {
    ( $err:ty, $src:ty, $dst:expr) => (
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

cast_err!(Error, io::Error, Error::Io);
cast_err!(Error, net::AddrParseError, Error::ParseAddr);
cast_err!(Error, num::ParseIntError, Error::ParseInt);

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

pub mod msg {
    const MSG_HEADER: [u8; 4] = [84, 84, 78, 83];

    pub struct Message {
        header: [u8; 4],    
        kind:   MessageKind,
    }

    pub enum MessageKind {
        IDENT,
    }

    impl Message {
        pub fn new(kind: MessageKind) -> Message {
            Message { header: MSG_HEADER, kind }
        }

        pub fn as_bytes(&self) -> [u8; 128] {
            let mut buf: [u8; 128] = [0; 128];

            for (i, n) in self.header.iter().enumerate() { buf[i] = *n };

            buf[4] = match &self.kind {
                MessageKind::IDENT => 0
            };

            buf
        }
    }
}