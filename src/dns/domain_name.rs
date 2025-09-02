use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainName(String);

/// Errors that can occur while parsing/validating a domain name.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum DomainNameError {
    #[error("empty label not allowed")]
    EmptyLabel,
    #[error("label too long: {0} octets (max 63)")]
    LabelTooLong(usize),
    #[error("name too long: {0} octets (max 255)")]
    NameTooLong(usize),
}

impl DomainName {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        // Labels are encoded as <length><content>, where <length> is a single byte that specifies the length of the label,
        // and <content> is the actual content of the label. The sequence of labels is terminated by a null byte (\x00).
        let name = &self.0;
        let mut bytes = Vec::with_capacity(name.len() + name.matches('.').count() + 1);
        for label in name.split('.') {
            let label_bytes = label.as_bytes();
            bytes.push(label_bytes.len() as u8);
            bytes.extend_from_slice(label_bytes);
        }
        bytes.push(0x00);
        bytes
    }
}

impl FromStr for DomainName {
    type Err = DomainNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // RFC 1035: each label <= 63 octets, total <= 255 (including length bytes + trailing zero)
        let mut total = 1; // account for the final 0 byte
        for label in s.split('.') {
            let len = label.len();
            if len == 0 {
                return Err(DomainNameError::EmptyLabel);
            }
            if len > 63 {
                return Err(DomainNameError::LabelTooLong(len));
            }
            total += 1 + len; // one length octet + label
        }
        if total > 255 {
            return Err(DomainNameError::NameTooLong(total));
        }
        Ok(DomainName(s.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_empty_name() {
        assert_eq!("".parse::<DomainName>(), Err(DomainNameError::EmptyLabel));
    }

    #[test]
    fn reject_too_long_label() {
        assert_eq!(
            "a".repeat(64).parse::<DomainName>(),
            Err(DomainNameError::LabelTooLong(64))
        );
    }

    #[test]
    fn reject_too_long_name() {
        assert_eq!(
            // "aaaaa." repeated 50 times
            std::iter::repeat("aaaaa")
                .take(50)
                .collect::<Vec<_>>()
                .join(".")
                .parse::<DomainName>(),
            Err(DomainNameError::NameTooLong(301))
        );
    }

    #[test]
    fn valid_domain_name() {
        let domain = DomainName::from_str("google.com").unwrap();
        let bytes = domain.to_be_bytes();
        insta::assert_snapshot!(format!("{:02x?}", &bytes), @"[06, 67, 6f, 6f, 67, 6c, 65, 03, 63, 6f, 6d, 00]");
    }
}
