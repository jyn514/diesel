use mysql::Mysql;
use query_builder::locking_clause::{ForShare, ForUpdate, NoModifier, NoWait, SkipLocked};
use query_builder::{AstPass, QueryFragment};
use result::QueryResult;

// impl QueryFragment for ForUpdate {
//     fn walk_ast(&self, mut out: AstPass) -> QueryResult {
//         out.push_sql(" FOR UPDATE");
//         Ok(())
//     }
// }

// impl QueryFragment for ForShare {
//     fn walk_ast(&self, mut out: AstPass) -> QueryResult {
//         out.push_sql(" FOR SHARE");
//         Ok(())
//     }
// }

// impl QueryFragment for NoModifier {
//     fn walk_ast(&self, _out: AstPass) -> QueryResult {
//         Ok(())
//     }
// }

// impl QueryFragment for SkipLocked {
//     fn walk_ast(&self, mut out: AstPass) -> QueryResult {
//         out.push_sql(" SKIP LOCKED");
//         Ok(())
//     }
// }

// impl QueryFragment for NoWait {
//     fn walk_ast(&self, mut out: AstPass) -> QueryResult {
//         out.push_sql(" NOWAIT");
//         Ok(())
//     }
// }
