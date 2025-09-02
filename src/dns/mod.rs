mod classes;
mod domain_name;
mod flags;
mod header;
mod message;
mod question;
mod resource_record;
mod types;

pub use classes::QClass;
pub use classes::RClass;
pub use domain_name::DomainName;
pub use message::Message;
pub use question::Question;
pub use resource_record::ResourceRecord;
pub use types::QType;
pub use types::RType;

pub struct Response;
pub struct Query;
