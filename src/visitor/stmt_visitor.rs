use crate::ast::stmt::Stmt;

pub trait StmtVisitor {
    fn visit_paraan(&mut self, paraan: &Stmt);
    fn visit_ang(&mut self, ang: &Stmt);
    fn visit_ibalik(&mut self, ibalik: &Stmt);
}
