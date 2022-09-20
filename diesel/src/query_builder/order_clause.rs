simple_clause!(NoOrderClause, OrderClause, " ORDER BY ");

// impl<'a, DB, Expr> Into<Option<Box<dyn QueryFragment + 'a>>> for OrderClause<Expr>
// where
//     DB: Backend,
//     Expr: QueryFragment + 'a,
// {
//     fn into(self) -> Option<Box<dyn QueryFragment + 'a>> {
//         Some(Box::new(self.0))
//     }
// }

// impl<'a, DB> Into<Option<Box<dyn QueryFragment + 'a>>> for NoOrderClause
// where
//     DB: Backend,
// {
//     fn into(self) -> Option<Box<dyn QueryFragment + 'a>> {
//         None
//     }
// }
