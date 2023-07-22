use std::fmt;

// Used for printing a value as hex when using the Debug trait
pub struct HexValue {
    pub value: u32,
}

pub trait HexValueExt {
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
