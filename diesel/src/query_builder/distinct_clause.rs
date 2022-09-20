use backend::Backend;
use query_builder::*;
use result::QueryResult;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoDistinctClause;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct DistinctClause;

impl<DB: Backend> QueryFragment for NoDistinctClause {
    fn walk_ast(&self, _: AstPass) -> QueryResult<()> {
        Ok(())
    }
}

impl<DB: Backend> QueryFragment for DistinctClause {
    fn walk_ast(&self, mut out: AstPass) -> QueryResult<()> {
        out.push_sql("DISTINCT ");
        Ok(())
    }
}

#[cfg(feature = "postgres")]
pub use pg::DistinctOnClause;
