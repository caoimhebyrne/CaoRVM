use crate::{ConstantPoolTag, HexValue};

pub type ClassParserResult<T> = Result<T, ClassParserError>;

#[derive(Debug)]
pub enum ClassParserError {
    InvalidMagic(HexValue),
    UnknownConstantTag(ConstantPoolTag),
    FailedToRead,
    UnexpectedEOF,
}

impl<V> From<ClassParserError> for Result<V, ClassParserError> {
    fn from(error: ClassParserError) -> Self {
        Err(error)
    }
}
