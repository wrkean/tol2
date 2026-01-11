use std::{fmt, ops::Range};

use crate::error::CompilerError;

/// Holds the primitves and user-defined types
#[derive(Debug, Clone, PartialEq)]
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

    F32,
    F64,

    Byte,
    Char,
    Bool,

    // Composites
    UnknownIdentifier(String),

    // Special
    Void,
    UnsizedInteger,
    UnsizedFloat,
}

impl TolType {
    pub fn coerce(&self, other: &TolType) -> Option<TolType> {
        use TolType::*;

        if self == other {
            return Some(self.clone());
        }

        match (self, other) {
            // signed ints
            (I8, I16) | (I16, I8) => Some(I16),
            (I8, I32) | (I32, I8) => Some(I32),
            (I8, I64) | (I64, I8) => Some(I64),
            (I16, I32) | (I32, I16) => Some(I32),
            (I16, I64) | (I64, I16) => Some(I64),
            (I32, I64) | (I64, I32) => Some(I64),

            // unsigned ints
            (U8, U16) | (U16, U8) => Some(U16),
            (U8, U32) | (U32, U8) => Some(U32),
            (U8, U64) | (U64, U8) => Some(U64),
            (U16, U32) | (U32, U16) => Some(U32),
            (U16, U64) | (U64, U16) => Some(U64),
            (U32, U64) | (U64, U32) => Some(U64),

            // int -> float
            (I8 | I16 | I32, F32) | (F32, I8 | I16 | I32) => Some(F32),

            (I8 | I16 | I32 | I64, F64) | (F64, I8 | I16 | I32 | I64) => Some(F64),

            (F32, F64) | (F64, F32) => Some(F64),

            // unsized integer
            (I8, UnsizedInteger) | (UnsizedInteger, I8) => Some(I8),
            (I16, UnsizedInteger) | (UnsizedInteger, I16) => Some(I16),
            (I32, UnsizedInteger) | (UnsizedInteger, I32) => Some(I32),
            (I64, UnsizedInteger) | (UnsizedInteger, I64) => Some(I64),
            (ISize, UnsizedInteger) | (UnsizedInteger, ISize) => Some(ISize),
            (U8, UnsizedInteger) | (UnsizedInteger, U8) => Some(U8),
            (U16, UnsizedInteger) | (UnsizedInteger, U16) => Some(U16),
            (U32, UnsizedInteger) | (UnsizedInteger, U32) => Some(U32),
            (U64, UnsizedInteger) | (UnsizedInteger, U64) => Some(U64),
            (USize, UnsizedInteger) | (UnsizedInteger, USize) => Some(USize),

            // unsized float
            (F32, UnsizedFloat) => Some(F32),
            (F64, UnsizedFloat) => Some(F64),

            _ => None,
        }
    }

    pub fn coerce_or_mismatch(
        &self,
        other: &TolType,
        self_span: Range<usize>,
        other_span: Range<usize>,
    ) -> Result<TolType, CompilerError> {
        self.coerce(other).ok_or(CompilerError::TypeMismatch {
            lhs_type: self.to_string(),
            rhs_type: other.to_string(),
            lhs_span: self_span.into(),
            rhs_span: other_span.into(),
        })
    }

    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            TolType::U8
                | TolType::U16
                | TolType::U32
                | TolType::U64
                | TolType::USize
                | TolType::I8
                | TolType::I16
                | TolType::I32
                | TolType::I64
                | TolType::ISize
                | TolType::F32
                | TolType::F64
                | TolType::UnsizedInteger
                | TolType::UnsizedFloat
        )
    }
}

impl fmt::Display for TolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TolType::U8 => write!(f, "u8"),
            TolType::U16 => write!(f, "u16"),
            TolType::U32 => write!(f, "u32"),
            TolType::U64 => write!(f, "u64"),
            TolType::USize => write!(f, "usize"),

            TolType::I8 => write!(f, "i8"),
            TolType::I16 => write!(f, "i16"),
            TolType::I32 => write!(f, "i32"),
            TolType::I64 => write!(f, "i64"),
            TolType::ISize => write!(f, "isize"),

            TolType::F32 => write!(f, "f32"),
            TolType::F64 => write!(f, "f64"),

            TolType::Byte => write!(f, "byte"),
            TolType::Char => write!(f, "char"),
            TolType::Bool => write!(f, "bool"),

            TolType::UnsizedInteger => write!(f, "UnsizedInteger"),
            TolType::UnsizedFloat => write!(f, "UnsizedFloat"),
            _ => panic!("Unrecognized string -> toltype!"),
        }
    }
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
