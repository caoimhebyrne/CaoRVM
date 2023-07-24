pub mod class_file;
pub use class_file::*;

pub mod error;
pub use error::*;

use crate::HexValueExt;

pub struct ClassFileParser {
    data: Vec<u8>,
}

impl ClassFileParser {
    pub fn new(data: Vec<u8>) -> ClassFileParser {
        ClassFileParser { data }
    }

    pub fn parse(&mut self) -> ClassParserResult<ClassFile> {
        let file = ClassFile {
            magic: self.read_u4()?.hex(),
            minor_version: self.read_u2()?,
            major_version: self.read_u2()?,
            constant_pool_count: self.read_u2()?,
        };

        Ok(file)
    }

    // A u2 in the JVM Class File spec is the same as a u16 in Rust.
    fn read_u2(&mut self) -> ClassParserResult<u16> {
        let array: Vec<u8> = self.read_n_bytes(2)?;
        let bytes = array
            .try_into()
            .map_err(|_| ClassParserError::FailedToRead)?;

        Ok(u16::from_be_bytes(bytes))
    }

    // A u4 in the JVM Class File spec is the same as a u32 in Rust.
    fn read_u4(&mut self) -> ClassParserResult<u32> {
        let array: Vec<u8> = self.read_n_bytes(4)?;
        let bytes = array
            .try_into()
            .map_err(|_| ClassParserError::FailedToRead)?;

        Ok(u32::from_be_bytes(bytes))
    }

    // Takes a certain amount of bytes out of the file
    fn read_n_bytes(&mut self, bytes: usize) -> Result<Vec<u8>, ClassParserError> {
        if self.data.is_empty() || bytes > self.data.len() {
            return ClassParserError::UnexpectedEOF.into();
        }

        Ok(self.data.drain(0..bytes).collect())
    }
}
