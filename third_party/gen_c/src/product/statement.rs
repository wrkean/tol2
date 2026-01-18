use crate::ctype::CType;

pub enum CStatement {
    Declaration {
        modifier: Option<String>,
        ttype: CType,
        name: String,
        rhs: Option<String>,
    },
    Function {
        modifiers: Vec<String>,
        return_type: CType,
        name: String,
        params: Vec<String>,
        body: Box<CStatement>,
    },
    Block {
        statements: Vec<CStatement>,
    },
}

impl CStatement {
    pub fn produce_c(self, indent: usize) -> String {
        match self {
            Self::Declaration {
                modifier,
                ttype,
                name,
                rhs,
            } => {
                format!(
                    "{}{} {}{}{}",
                    " ".repeat(indent),
                    ttype,
                    modifier.map_or("".to_string(), |s| s.to_string() + " "),
                    name,
                    rhs.map_or(";".to_string(), |e| format!(" = {};", e))
                )
            }
            Self::Function {
                modifiers,
                return_type,
                name,
                params,
                body,
            } => {
                let body = if let CStatement::Block { statements } = *body {
                    if statements.is_empty() {
                        ";".to_string()
                    } else {
                        format!(
                            " {{
{}
}}",
                            statements
                                .into_iter()
                                .map(|s| s.produce_c(indent + 4))
                                .collect::<Vec<_>>()
                                .join("\n")
                        )
                    }
                } else {
                    unreachable!()
                };
                format!(
                    "{}{} {}({}){}",
                    if modifiers.is_empty() {
                        "".to_string()
                    } else {
                        modifiers.join(" ")
                    },
                    return_type,
                    name,
                    params.join(", "),
                    body,
                )
            }
            Self::Block { statements } => format!(
                "{}{{
{}
{}}}",
                " ".repeat(indent),
                statements
                    .into_iter()
                    .map(|cs| cs.produce_c(indent + 4))
                    .collect::<Vec<_>>()
                    .join("\n"),
                " ".repeat(indent)
            ),
        }
    }
}
