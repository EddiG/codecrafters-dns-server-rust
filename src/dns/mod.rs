mod flags;
mod header;
mod message;
mod question;

pub use message::Message;
pub use question::QClass;
pub use question::QType;
pub use question::Question;

pub struct Response;
pub struct Query;
