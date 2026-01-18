pub enum CExpr {
    FromStr(String),
}

impl CExpr {
    pub fn produce_c(self) -> String {
        match self {
            Self::FromStr(s) => s,
        }
    }

    pub fn from_string(strn: &str) -> CExpr {
        Self::FromStr(strn.to_string())
    }
}
