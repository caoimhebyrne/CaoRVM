use std::fmt;

use crate::{ClassFileParser, ClassParserError, ClassParserResult};

// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4-140
#[derive(Debug)]
pub enum ConstantPoolTag {
    Class = 7,
    FieldReference = 9,
    MethodReference = 10,
    InterfaceMethodReference = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

impl From<u8> for ConstantPoolTag {
    fn from(value: u8) -> ConstantPoolTag {
        match value {
            7 => ConstantPoolTag::Class,
            9 => ConstantPoolTag::FieldReference,
            10 => ConstantPoolTag::MethodReference,
            11 => ConstantPoolTag::InterfaceMethodReference,
            8 => ConstantPoolTag::String,
            3 => ConstantPoolTag::Integer,
            4 => ConstantPoolTag::Float,
            5 => ConstantPoolTag::Long,
            6 => ConstantPoolTag::Double,
            12 => ConstantPoolTag::NameAndType,
            1 => ConstantPoolTag::Utf8,
            15 => ConstantPoolTag::MethodHandle,
            16 => ConstantPoolTag::MethodType,
            17 => ConstantPoolTag::Dynamic,
            18 => ConstantPoolTag::InvokeDynamic,
            19 => ConstantPoolTag::Module,
            20 => ConstantPoolTag::Package,
            _ => panic!("Unknown constant pool tag {}", value),
        }
    }
}

// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4
pub trait ConstantPoolEntry: fmt::Debug {
    fn parse(parser: &mut ClassFileParser, tag: ConstantPoolTag) -> ClassParserResult<Self>
    where
        Self: Sized;
}

// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.2
#[derive(Debug)]
pub enum ReferenceType {
    Field,
    Method,
    InterfaceMethod,
}

#[derive(Debug)]
pub struct ReferenceConstant {
    reference_type: ReferenceType,
    class_index: u16,
    name_and_type_index: u16,
}

impl ConstantPoolEntry for ReferenceConstant {
    fn parse(
        parser: &mut ClassFileParser,
        tag: ConstantPoolTag,
    ) -> ClassParserResult<ReferenceConstant> {
        let reference_type = match tag {
            ConstantPoolTag::FieldReference => ReferenceType::Field,
            ConstantPoolTag::MethodReference => ReferenceType::Method,
            ConstantPoolTag::InterfaceMethodReference => ReferenceType::InterfaceMethod,
            _ => return ClassParserError::UnknownConstantTag(tag).into(),
        };

        let class_index = parser.read_u2()?;
        let name_and_type_index = parser.read_u2()?;

        Ok(ReferenceConstant {
            reference_type,
            class_index,
            name_and_type_index,
        })
    }
}

#[derive(Debug)]
pub struct ClassConstant {
    name_index: u16,
}

impl ConstantPoolEntry for ClassConstant {
    fn parse(
        parser: &mut ClassFileParser,
        _tag: ConstantPoolTag,
    ) -> ClassParserResult<ClassConstant> {
        let name_index: u16 = parser.read_u2()?;
        Ok(ClassConstant { name_index })
    }
}

// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.6
#[derive(Debug)]
pub struct NameAndTypeConstant {
    name_index: u16,
    descriptor_index: u16,
}

impl ConstantPoolEntry for NameAndTypeConstant {
    fn parse(
        parser: &mut ClassFileParser,
        _tag: ConstantPoolTag,
    ) -> ClassParserResult<NameAndTypeConstant> {
        let name_index: u16 = parser.read_u2()?;
        let descriptor_index: u16 = parser.read_u2()?;

        Ok(NameAndTypeConstant {
            name_index,
            descriptor_index,
        })
    }
}

// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.7
#[derive(Debug)]
pub struct UTF8Constant {
    data: String,
}

impl ConstantPoolEntry for UTF8Constant {
    fn parse(
        parser: &mut ClassFileParser,
        _tag: ConstantPoolTag,
    ) -> ClassParserResult<UTF8Constant> {
        let length = parser.read_u2()?;
        let bytes = parser.read_n_bytes(length.into())?;
        let data = String::from_utf8(bytes).map_err(|_| ClassParserError::FailedToRead)?;

        Ok(UTF8Constant { data })
    }
}

// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.3
#[derive(Debug)]
pub struct StringConstant {
    string_index: u16,
}

impl ConstantPoolEntry for StringConstant {
    fn parse(
        parser: &mut ClassFileParser,
        _tag: ConstantPoolTag,
    ) -> ClassParserResult<StringConstant> {
        let string_index = parser.read_u2()?;
        Ok(StringConstant { string_index })
    }
}
