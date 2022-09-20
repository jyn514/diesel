macro_rules! simple_clause {
    ($no_clause:ident, $clause:ident, $sql:expr) => {
        simple_clause!($no_clause, $clause, $sql, backend_bounds = );
    };

    ($no_clause:ident, $clause:ident, $sql:expr, backend_bounds = $($backend_bounds:ident),*) => {
        use backend::Backend;
        use result::QueryResult;
        use super::{QueryFragment, AstPass};

        #[derive(Debug, Clone, Copy, QueryId)]
        pub struct $no_clause;

        // impl<DB: Backend> QueryFragment<DB> for $no_clause {
        //     fn walk_ast(&self, _: AstPass) -> QueryResult<()> {
        //         Ok(())
        //     }
        // }

        #[derive(Debug, Clone, Copy, QueryId)]
        pub struct $clause<Expr>(pub Expr);

        // impl<Expr, DB> QueryFragment<DB> for $clause<Expr> where
        //     DB: Backend $(+ $backend_bounds)*,
        //     Expr: QueryFragment<DB>,
        // {
        //     fn walk_ast(&self, mut out: AstPass) -> QueryResult<()> {
        //         out.push_sql($sql);
        //         self.0.walk_ast(out.reborrow())?;
        //         Ok(())
        //     }
        // }
    }
}
