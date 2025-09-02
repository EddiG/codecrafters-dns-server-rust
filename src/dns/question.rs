use super::{classes::QClass, domain_name::DomainName, types::QType};

#[derive(Debug, Clone)]
pub struct Question {
    qname: DomainName,
    qtype: QType,
    qclass: QClass,
}

impl Question {
    pub fn new(qname: DomainName, qtype: QType, qclass: QClass) -> Self {
        Self {
            qname,
            qtype,
            qclass,
        }
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let qname_bytes = self.qname.to_be_bytes();
        let qtype_bytes = self.qtype.to_be_bytes();
        let qclass_bytes = self.qclass.to_be_bytes();
        let mut bytes =
            Vec::with_capacity(qname_bytes.len() + qtype_bytes.len() + qclass_bytes.len());
        bytes.extend_from_slice(&qname_bytes);
        bytes.extend_from_slice(&qtype_bytes);
        bytes.extend_from_slice(&qclass_bytes);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_be_bytes() {
        let domain = "google.com".parse::<DomainName>().unwrap();
        let question = Question::new(domain, QType::CNAME, QClass::IN);
        let bytes = question.to_be_bytes();
        insta::assert_snapshot!(format!("{:02x?}", &bytes), @"[06, 67, 6f, 6f, 67, 6c, 65, 03, 63, 6f, 6d, 00, 00, 05, 00, 01]");
    }
}
