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
    }
}