use super::{header::Header, resource_record::ResourceRecord, Question, Response};

/// The DNS protocol uses two types of DNS messages, queries and responses; both have the same format.
/// Each message consists of a header and four sections: question, answer, authority, and an additional space.
/// A header field (flags) controls the content of these four sections.
#[derive(Debug, Clone)]
pub struct Message<Dir> {
    header: Header<Dir>,
    question: Vec<Question>,
    answer: Vec<ResourceRecord>,
    // authority: Authority,
    // addition: Addition,
}

impl Message<Response> {
    /// Create a new response message with the given ID.
    pub fn response(id: u16, question: Vec<Question>, answer: Vec<ResourceRecord>) -> Self {
        let mut header = Header::response(id);
        header.qdcount = question.len() as u16;
        header.ancount = answer.len() as u16;

        Self {
            header,
            question,
            answer,
        }
    }
}

impl<Dir> Message<Dir> {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let header_bytes = self.header.to_be_bytes();
        let question_bytes = self.question.iter().flat_map(|q| q.to_be_bytes());
        let answer_bytes = self.answer.iter().flat_map(|a| a.to_be_bytes());
        header_bytes
            .into_iter()
            .chain(question_bytes)
            .chain(answer_bytes)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dns::{DomainName, QClass, QType, RClass, RType};

    #[test]
    fn valid_empty_response_message() {
        let message = Message::response(1, vec![], vec![]);
        let bytes = message.to_be_bytes();
        insta::assert_snapshot!(format!("{:02x?}", &bytes), @"[00, 01, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00]");
    }

    #[test]
    fn valid_one_question_response_message() {
        let domain = "google.com".parse::<DomainName>().unwrap();
        let question = Question::new(domain, QType::CNAME, QClass::IN);
        let message = Message::response(1, vec![question], vec![]);
        let bytes = message.to_be_bytes();
        insta::assert_snapshot!(format!("{:02x?}", &bytes), @"[00, 01, 80, 00, 00, 01, 00, 00, 00, 00, 00, 00, 06, 67, 6f, 6f, 67, 6c, 65, 03, 63, 6f, 6d, 00, 00, 05, 00, 01]");
    }

    #[test]
    fn valid_question_and_answer_response_message() {
        let domain = "google.com".parse::<DomainName>().unwrap();
        let question = Question::new(domain.clone(), QType::CNAME, QClass::IN);
        let answer = ResourceRecord::new(
            domain,
            RType::CNAME,
            RClass::IN,
            3600,
            vec![0x08, 0x08, 0x08, 0x08],
        );
        let message = Message::response(1, vec![question], vec![answer]);
        let bytes = message.to_be_bytes();
        insta::assert_snapshot!(format!("{:02x?}", &bytes), @"[00, 01, 80, 00, 00, 01, 00, 01, 00, 00, 00, 00, 06, 67, 6f, 6f, 67, 6c, 65, 03, 63, 6f, 6d, 00, 00, 05, 00, 01, 06, 67, 6f, 6f, 67, 6c, 65, 03, 63, 6f, 6d, 00, 00, 05, 00, 01, 00, 00, 0e, 10, 08, 08, 08, 08]");
    }
}
