pub mod class_file;
pub use class_file::*;

pub mod constant_pool;
pub use constant_pool::*;

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
        let magic = self.read_u4()?.hex();
        let minor_version = self.read_u2()?;
        let major_version = self.read_u2()?;

        let constant_pool_length = self.read_u2()?;

        let mut constant_pool = vec![];
        for _ in 0..constant_pool_length - 1 {
            let entry = self.read_constant_pool_info()?;
            constant_pool.push(entry);
        }

        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool,
        })
    }

    pub fn read_constant_pool_info(&mut self) -> ClassParserResult<Box<dyn ConstantPoolEntry>> {
        let tag = self.read_u1()?.into();

        Ok(match tag {
            // All of the reference tags have the same data structure
            ConstantPoolTag::MethodReference
            | ConstantPoolTag::FieldReference
            | ConstantPoolTag::InterfaceMethodReference => {
                let constant = ReferenceConstant::parse(self, tag)?;
                Box::new(constant)
            }

            ConstantPoolTag::Class => {
                let constant = ClassConstant::parse(self, tag)?;
                Box::new(constant)
            }

            ConstantPoolTag::NameAndType => {
                let constant = NameAndTypeConstant::parse(self, tag)?;
                Box::new(constant)
            }

            ConstantPoolTag::Utf8 => {
                let constant = UTF8Constant::parse(self, tag)?;
                Box::new(constant)
            }

            ConstantPoolTag::String => {
                let constant = StringConstant::parse(self, tag)?;
                Box::new(constant)
            }

            _ => return ClassParserError::UnknownConstantTag(tag).into(),
        })
    }

    // A u1 in the JVM Class File spec is the same as a u8 in Rust.
    pub fn read_u1(&mut self) -> ClassParserResult<u8> {
        let array: Vec<u8> = self.read_n_bytes(1)?;
        let bytes = array
            .try_into()
            .map_err(|_| ClassParserError::FailedToRead)?;

        Ok(u8::from_be_bytes(bytes))
    }

    // A u2 in the JVM Class File spec is the same as a u16 in Rust.
    pub fn read_u2(&mut self) -> ClassParserResult<u16> {
        let array: Vec<u8> = self.read_n_bytes(2)?;
        let bytes = array
            .try_into()
            .map_err(|_| ClassParserError::FailedToRead)?;

        Ok(u16::from_be_bytes(bytes))
    }

    // A u4 in the JVM Class File spec is the same as a u32 in Rust.
    pub fn read_u4(&mut self) -> ClassParserResult<u32> {
        let array: Vec<u8> = self.read_n_bytes(4)?;
        let bytes = array
            .try_into()
            .map_err(|_| ClassParserError::FailedToRead)?;

        Ok(u32::from_be_bytes(bytes))
    }

    // Takes a certain amount of bytes out of the file
    pub fn read_n_bytes(&mut self, bytes: usize) -> Result<Vec<u8>, ClassParserError> {
        if self.data.is_empty() || bytes > self.data.len() {
            return ClassParserError::UnexpectedEOF.into();
        }

        Ok(self.data.drain(0..bytes).collect())
    }
}
