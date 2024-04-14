use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;


#[derive(Error, Debug)]
pub enum ReadError {
    #[error("failed to open the file: `{0}`")]
    FailedToOpenFile(String),

    #[error("invalid operation")]
    InvalidOperation,

    #[error("unknown attribute type: `{0}`")]
    UnknownAttributeType(String),

    #[error("footer2 has invalid byte pattern")]
    InvalidFooter2BytePattern,

    #[error("footer3 does not match to given bytes")]
    Footer3DoesNotMatch(Vec<u8>),

    #[error("footer4 does not match to given bytes")]
    Footer4DoesNotMatch(Vec<u8>),
}

#[derive(Error, Debug)]
pub enum WriteError {}

