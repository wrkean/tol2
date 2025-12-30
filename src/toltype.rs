/// Holds the primitves and user-defined types
#[derive(Debug)]
pub enum TolType {
    // Unsigned integers
    U8,
    U16,
    U32,
    U64,
    USize,

    // Signed integers
    I8,
    I16,
    I32,
    I64,
    ISize,

    Byte,
    Char,
    Bool,

    // Composites
    UnknownIdentifier(String),

    // Special
    Void,
}

impl From<&str> for TolType {
    fn from(value: &str) -> Self {
        match value {
            "u8" => TolType::U8,
            "u16" => TolType::U16,
            "u32" => TolType::U32,
            "u64" => TolType::U64,
            "usize" => TolType::USize,

            "i8" => TolType::I8,
            "i16" => TolType::I16,
            "i32" => TolType::I32,
            "i64" => TolType::I64,
            "isize" => TolType::ISize,

            "byte" => TolType::Byte,
            "char" => TolType::Char,
            "bool" => TolType::Bool,
            _ => panic!("Unrecognized string -> toltype!"),
        }
    }
}
