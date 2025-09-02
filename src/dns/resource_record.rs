use super::{classes::RClass, types::RType, DomainName};

#[derive(Debug, Clone)]
pub struct ResourceRecord {
    name: DomainName,
    rtype: RType,
    rclass: RClass,
    ttl: u32,
    length: u16,
    data: Vec<u8>,
}

impl ResourceRecord {
    pub fn new(name: DomainName, rtype: RType, rclass: RClass, ttl: u32, data: Vec<u8>) -> Self {
        ResourceRecord {
            name,
            rtype,
            rclass,
            ttl,
            length: data.len() as u16,
            data,
        }
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let name_bytes = self.name.to_be_bytes();
        let rtype_bytes = self.rtype.to_be_bytes();
        let rclass_bytes = self.rclass.to_be_bytes();
        let ttl_bytes = self.ttl.to_be_bytes();
        let data_bytes = &self.data;
        let length_bytes = self.length.to_be_bytes();
        let mut bytes = Vec::with_capacity(
            name_bytes.len()
                + rtype_bytes.len()
                + rclass_bytes.len()
                + ttl_bytes.len()
                + length_bytes.len()
                + data_bytes.len(),
        );
        bytes.extend_from_slice(&name_bytes);
        bytes.extend_from_slice(&rtype_bytes);
        bytes.extend_from_slice(&rclass_bytes);
        bytes.extend_from_slice(&ttl_bytes);
        bytes.extend_from_slice(&length_bytes);
        bytes.extend_from_slice(&data_bytes);
        bytes
    }
}
