use crate::ctype::CType;
use std::fmt::Write;

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
    Return {
        rhs: Option<String>,
    },
    While {
        cond: String,
        body: Box<CStatement>,
    },
    If {
        initial_cond: String,
        initial_block: Box<CStatement>,
        branches: Vec<IfBranch>,
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
            Self::Return { rhs } => format!(
                "{}return {};",
                " ".repeat(indent),
                rhs.unwrap_or("".to_string())
            ),

            Self::While { cond, body } => {
                format!(
                    "{}while ({})
{}",
                    " ".repeat(indent),
                    cond,
                    body.produce_c(indent)
                )
            }
            Self::If {
                initial_cond,
                initial_block,
                branches,
            } => {
                let mut initial_branch = format!(
                    "{}if ({})
{}",
                    " ".repeat(indent),
                    initial_cond,
                    initial_block.produce_c(indent)
                );

                for branch in branches {
                    let _ = match branch.cond {
                        Some(s) => write!(
                            initial_branch,
                            " else if ({})
{}",
                            s,
                            branch.body.produce_c(indent)
                        ),
                        None => write!(
                            initial_branch,
                            " else
{}",
                            branch.body.produce_c(indent)
                        ),
                    };
                }

                initial_branch
            }
        }
    }
}

pub struct IfBranch {
    pub(crate) cond: Option<String>,
    pub(crate) body: Box<CStatement>,
}

impl IfBranch {
    pub fn new(cond: Option<String>, body: CStatement) -> Self {
        Self {
            cond,
            body: Box::new(body),
        }
    }

    pub fn cond(&self) -> Option<&String> {
        self.cond.as_ref()
    }

    pub fn body(&self) -> &CStatement {
        &self.body
    }
}
