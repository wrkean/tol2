#[derive(Clone, Copy)]
pub enum ExprParseContext {
    AngDapatStatement,
    KungStatement,
    HabangStatement,
    BawatStatement,
    InExpression,
    Argument,
    StructLiteralField,
}

impl ExprParseContext {
    pub fn can_have_struct_lit(&self) -> bool {
        matches!(self, Self::AngDapatStatement | Self::StructLiteralField)
    }
}
