use super::header::Header;

/// The DNS protocol uses two types of DNS messages, queries and responses; both have the same format.
/// Each message consists of a header and four sections: question, answer, authority, and an additional space.
/// A header field (flags) controls the content of these four sections.
#[derive(Debug, Clone, Copy)]
pub struct Message {
    header: Header,
    // question: Question,
    // answer: Answer,
    // authority: Authority,
    // addition: Addition,
}

impl Message {
    pub fn new() -> Self {
        Message {
            header: Header::response(1234),
        }
    }

    /// Serialize into a 12-byte array (big-endian/u16 network order)
    pub fn to_bytes(&self) -> [u8; Header::SIZE] {
        self.header.to_bytes()
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}
