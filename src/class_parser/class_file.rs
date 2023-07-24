// https://docs.oracle.com/javase/specs/jvms/se8/html/

use crate::{ConstantPoolEntry, HexValue};

type ConstantPool = Vec<Box<dyn ConstantPoolEntry>>;

#[derive(Debug)]
pub struct ClassFile {
    pub magic: HexValue,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
}
