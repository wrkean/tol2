use crate::ast::expr::Expr;

pub trait ExprVisitor {
    fn visit_integer(&mut self, integer: &Expr);
    fn visit_float(&mut self, float: &Expr);
    fn visit_boolean(&mut self, boolean: &Expr);
    fn visit_add(&mut self, add: &Expr);
}
