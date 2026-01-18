use std::fmt;

pub enum CType {
    I8,
    I16,
    I32,
    I64,
    PtrDiff,
    U8,
    U16,
    U32,
    U64,
    Size, // size_t
    Bool,
    Char,
    Custom(String),
}

impl fmt::Display for CType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CType::I8 => write!(f, "int8_t"),
            CType::I16 => write!(f, "int16_t"),
            CType::I32 => write!(f, "int32_t"),
            CType::I64 => write!(f, "int64_t"),
            CType::PtrDiff => write!(f, "ptrdiff_t"),
            CType::U8 => write!(f, "uint8_t"),
            CType::U16 => write!(f, "uint16_t"),
            CType::U32 => write!(f, "uint32_t"),
            CType::U64 => write!(f, "uint64_t"),
            CType::Size => write!(f, "size_t"),
            CType::Bool => write!(f, "bool"),
            CType::Char => write!(f, "char"),
            CType::Custom(s) => write!(f, "{s}"),
        }
    }
}
