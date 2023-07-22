use std::{fmt, fs};

// Used for printing a value as hex when using the Debug trait
struct HexValue {
    value: u32,
}

trait HexValueExt {
    fn hex(self) -> HexValue;
}

impl HexValueExt for u32 {
    fn hex(self) -> HexValue {
        HexValue { value: self }
    }
}

impl fmt::Debug for HexValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#02X}", self.value)
    }
}

// https://docs.oracle.com/javase/specs/jvms/se8/html/
#[derive(Debug)]
struct ClassFile {
    magic: HexValue,
    minor_version: u16,
    major_version: u16,

    constant_pool_count: u16,
}

#[derive(Debug)]
enum ClassParserError {
    InvalidMagic(HexValue),
    FailedToRead,
    UnexpectedEOF,
}

impl<V> From<ClassParserError> for Result<V, ClassParserError> {
    fn from(error: ClassParserError) -> Self {
        Err(error)
    }
}

fn parse_class_file(file: &mut Vec<u8>) -> Result<ClassFile, ClassParserError> {
    let magic = read_u4(file)?;
    if magic != 0xCAFEBABE {
        return ClassParserError::InvalidMagic(magic.hex()).into();
    }

    let minor_version = read_u2(file)?;
    let major_version = read_u2(file)?;

    let constant_pool_count = read_u2(file)?;

    let file = ClassFile {
        magic: magic.hex(),
        minor_version,
        major_version,
        constant_pool_count,
    };

    Ok(file)
}

fn main() {
    let mut file = fs::read("./tests/Main.class").unwrap();

    match parse_class_file(&mut file) {
        Ok(parsed) => {
            if parsed.major_version != 52 || parsed.minor_version != 0 {
                eprintln!(
                    "Expected major version to be 52 (got {}) and minor version to be 0 (got {})!",
                    parsed.major_version, parsed.minor_version
                );
            }

            println!("Parsed class file: {:?}", parsed)
        }
        Err(error) => eprintln!("Failed to parse class file: {:?}", error),
    }
}

fn read_u1(file: &mut Vec<u8>) -> Result<u8, ClassParserError> {
    if file.is_empty() {
        return ClassParserError::UnexpectedEOF.into();
    }

    let array: Vec<u8> = read_n_bytes(file, 1);
    match array.try_into() {
        Ok(value) => Ok(u8::from_be_bytes(value)),
        Err(_) => ClassParserError::FailedToRead.into(),
    }
}

fn read_u2(file: &mut Vec<u8>) -> Result<u16, ClassParserError> {
    if file.is_empty() {
        return ClassParserError::UnexpectedEOF.into();
    }

    let array: Vec<u8> = read_n_bytes(file, 2);
    match array.try_into() {
        Ok(value) => Ok(u16::from_be_bytes(value)),
        Err(_) => ClassParserError::FailedToRead.into(),
    }
}

fn read_u4(file: &mut Vec<u8>) -> Result<u32, ClassParserError> {
    if file.is_empty() {
        return ClassParserError::UnexpectedEOF.into();
    }

    let array: Vec<u8> = read_n_bytes(file, 4);
    match array.try_into() {
        Ok(value) => Ok(u32::from_be_bytes(value)),
        Err(_) => ClassParserError::FailedToRead.into(),
    }
}

fn read_n_bytes(file: &mut Vec<u8>, bytes: usize) -> Vec<u8> {
    if file.is_empty() || bytes > file.len() {
        return vec![];
    }

    file.drain(0..bytes).collect()
}
