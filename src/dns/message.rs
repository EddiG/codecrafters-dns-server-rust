use super::{header::Header, Question, Response};

/// The DNS protocol uses two types of DNS messages, queries and responses; both have the same format.
/// Each message consists of a header and four sections: question, answer, authority, and an additional space.
/// A header field (flags) controls the content of these four sections.
#[derive(Debug, Clone)]
pub struct Message<Dir> {
    header: Header<Dir>,
    question: Vec<Question>,
    // answer: Answer,
    // authority: Authority,
    // addition: Addition,
}

impl Message<Response> {
    /// Create a new response message with the given ID.
    pub fn response(id: u16, question: Vec<Question>) -> Self {
        let mut header = Header::response(id);
        header.qdcount = question.len() as u16;

        Self { header, question }
    }
}

impl<Dir> Message<Dir> {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let header_bytes = self.header.to_be_bytes();
        let question_bytes = self.question.iter().flat_map(|q| q.to_be_bytes());
        header_bytes.into_iter().chain(question_bytes).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_response_message_to_be_bytes() {
        let message = Message::response(1, vec![]);
        let bytes = message.to_be_bytes();
        insta::assert_snapshot!(format!("{:02x?}", &bytes), @"[00, 01, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00]");
    }

    #[test]
    fn test_response_message_to_be_bytes() {
        let message = Message::response(1, vec![Question::new("google.com", 5, 1)]);
        let bytes = message.to_be_bytes();
        insta::assert_snapshot!(format!("{:02x?}", &bytes), @"[00, 01, 80, 00, 00, 01, 00, 00, 00, 00, 00, 00, 06, 67, 6f, 6f, 67, 6c, 65, 03, 63, 6f, 6d, 00, 00, 05, 00, 01]");
    }
}
