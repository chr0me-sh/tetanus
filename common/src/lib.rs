use std::net;

mod msg {
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