#[derive(Debug, Clone, Copy)]
pub enum QType {
    A = 1,
    CNAME = 5,
}

impl QType {
    pub fn to_be_bytes(&self) -> [u8; 2] {
        (*self as u16).to_be_bytes()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QClass {
    IN = 1,
}

impl QClass {
    pub fn to_be_bytes(&self) -> [u8; 2] {
        (*self as u16).to_be_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct Question {
    qname: Vec<u8>,
    qtype: QType,
    qclass: QClass,
}

impl Question {
    pub fn new(name: &str, qtype: QType, qclass: QClass) -> Self {
        // Labels are encoded as <length><content>, where <length> is a single byte that specifies the length of the label,
        // and <content> is the actual content of the label. The sequence of labels is terminated by a null byte (\x00).
        let mut qname = Vec::with_capacity(name.len() + name.matches('.').count() + 1);
        for label in name.split('.') {
            let bytes = label.as_bytes();
            qname.push(bytes.len() as u8);
            qname.extend_from_slice(bytes);
        }
        qname.push(0x00);

        Self {
            qname,
            qtype,
            qclass,
        }
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.qname.len() + 4);
        bytes.extend_from_slice(&self.qname);
        bytes.extend_from_slice(&self.qtype.to_be_bytes());
        bytes.extend_from_slice(&self.qclass.to_be_bytes());
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_be_bytes() {
        let question = Question::new("google.com", QType::CNAME, QClass::IN);
        let bytes = question.to_be_bytes();
        assert_eq!(
            bytes,
            [
                0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, // length + "google"
                0x03, 0x63, 0x6f, 0x6d, // length + "com"
                0x00, // null
                0x00, 0x05, // type
                0x00, 0x01 // class
            ]
        );
    }
}
