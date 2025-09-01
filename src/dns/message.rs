use super::{header::Header, Response};

/// The DNS protocol uses two types of DNS messages, queries and responses; both have the same format.
/// Each message consists of a header and four sections: question, answer, authority, and an additional space.
/// A header field (flags) controls the content of these four sections.
#[derive(Debug, Clone, Copy)]
pub struct Message<Dir> {
    header: Header<Dir>,
    // question: Question,
    // answer: Answer,
    // authority: Authority,
    // addition: Addition,
}

impl Message<Response> {
    /// Create a new response message with the given ID.
    pub fn new(id: u16) -> Self {
        Message {
            header: Header::response(id),
        }
    }
}

impl<Dir> Message<Dir> {
    pub fn to_bytes(&self) -> [u8; 12] {
        self.header.to_bytes()
    }
}
