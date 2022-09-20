use backend::Backend;
use query_builder::*;
use result::QueryResult;

#[derive(Debug, Copy, Clone)]
pub struct Identifier<'a>(pub &'a str);

// impl<'a, DB: Backend> QueryFragment for Identifier<'a> {
//     fn walk_ast(&self, mut out: AstPass) -> QueryResult {
//         out.push_identifier(self.0)
//     }
// }

#[derive(Debug, Copy, Clone)]
pub struct InfixNode<'a, T, U> {
    lhs: T,
    rhs: U,
    middle: &'a str,
}

impl<'a, T, U> InfixNode<'a, T, U> {
    pub fn new(lhs: T, rhs: U, middle: &'a str) -> Self {
        InfixNode {
            lhs: lhs,
            rhs: rhs,
            middle: middle,
        }
    }
}

// impl<'a, T, U, DB> QueryFragment for InfixNode<'a, T, U>
// where
//     DB: Backend,
//     T: QueryFragment,
//     U: QueryFragment,
// {
//     fn walk_ast(&self, mut out: AstPass) -> QueryResult {
//         self.lhs.walk_ast(out.reborrow())?;
//         out.push_sql(self.middle);
//         self.rhs.walk_ast(out.reborrow())?;
//         Ok(())
//     }
// }
