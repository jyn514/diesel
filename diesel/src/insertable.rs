use query_builder::{
    // AstPass, InsertStatement, QueryFragment, ValuesClause,
    InsertStatement, QueryFragment,
};
use result::QueryResult;

pub struct ValuesClause<T, Tab> {
    pub(crate) values: T,
    _marker: PhantomData<Tab>,
}

// trait QueryFragment<DB: Backend> {
//     /// Walk over this `QueryFragment` for all passes.
//     ///
//     /// This method is where the actual behavior of an AST node is implemented.
//     /// This method will contain the behavior required for all possible AST
//     /// passes. See [`AstPass`] for more details.
//     ///
//     /// [`AstPass`]: struct.AstPass.html
//     fn walk_ast(&self, pass: AstPass<DB>) -> QueryResult<()>;

//     /// Converts this `QueryFragment` to its SQL representation.
//     ///
//     /// This method should only be called by implementations of `Connection`.
//     fn to_sql(&self, out: &mut DB::QueryBuilder) -> QueryResult<()> {
//         self.walk_ast(AstPass::to_sql(out))
//     }

//     /// Serializes all bind parameters in this query.
//     ///
//     /// A bind parameter is a value which is sent separately from the query
//     /// itself. It is represented in SQL with a placeholder such as `?` or `$1`.
//     ///
//     /// This method should only be called by implementations of `Connection`.
//     fn collect_binds(
//         &self,
//         out: &mut DB::BindCollector,
//         metadata_lookup: &DB::MetadataLookup,
//     ) -> QueryResult<()> {
//         self.walk_ast(AstPass::collect_binds(out, metadata_lookup))
//     }

//     /// Is this query safe to store in the prepared statement cache?
//     ///
//     /// In order to keep our prepared statement cache at a reasonable size, we
//     /// avoid caching any queries which represent a potentially unbounded number
//     /// of SQL queries. Generally this will only return `true` for queries for
//     /// which `to_sql` will always construct exactly identical SQL.
//     ///
//     /// Some examples of where this method will return `false` are:
//     ///
//     /// - `SqlLiteral` (We don't know if the SQL was constructed dynamically, so
//     ///   we must assume that it was)
//     /// - `In` and `NotIn` (Each value requires a separate bind param
//     ///   placeholder)
//     ///
//     /// This method should only be called by implementations of `Connection`.
//     fn is_safe_to_cache_prepared(&self) -> QueryResult<bool> {
//         let mut result = true;
//         self.walk_ast(AstPass::is_safe_to_cache_prepared(&mut result))?;
//         Ok(result)
//     }

//     #[doc(hidden)]
//     /// Does walking this AST have any effect?
//     fn is_noop(&self) -> QueryResult<bool> {
//         let mut result = true;
//         self.walk_ast(AstPass::is_noop(&mut result))?;
//         Ok(result)
//     }
// }
use std::marker::PhantomData;

use backend::Backend;
pub trait Insertable<T> {
    type Values;

    fn values(self) -> Self::Values;

    fn insert_into(self, table: T) -> InsertStatement<T, Self::Values>
    where
        Self: Sized,
    {
    }
}

// trait QueryFragment<DB: Backend> {
//     /// Walk over this `QueryFragment` for all passes.
//     ///
//     /// This method is where the actual behavior of an AST node is implemented.
//     /// This method will contain the behavior required for all possible AST
//     /// passes. See [`AstPass`] for more details.
//     //
//     /// [`AstPass`]: struct.AstPass.html
//     fn walk_ast(&self, pass: AstPass<DB>) -> QueryResult<()>;

//     /// Converts this `QueryFragment` to its SQL representation.
//     ///
//     /// This method should only be called by implementations of `Connection`.
//     fn to_sql(&self, out: &mut DB::QueryBuilder) -> QueryResult<()> {
//         self.walk_ast(AstPass::to_sql(out))
//     }

//     /// Serializes all bind parameters in this query.
//     ///
//     /// A bind parameter is a value which is sent separately from the query
//     /// itself. It is represented in SQL with a placeholder such as `?` or `$1`.
//     ///
//     /// This method should only be called by implementations of `Connection`.
//     fn collect_binds(
//         &self,
//         out: &mut DB::BindCollector,
//         metadata_lookup: &DB::MetadataLookup,
//     ) -> QueryResult<()> {
//         self.walk_ast(AstPass::collect_binds(out, metadata_lookup))
//     }

//     /// Is this query safe to store in the prepared statement cache?
//     ///
//     /// In order to keep our prepared statement cache at a reasonable size, we
//     /// avoid caching any queries which represent a potentially unbounded number
//     /// of SQL queries. Generally this will only return `true` for queries for
//     /// which `to_sql` will always construct exactly identical SQL.
//     ///
//     /// Some examples of where this method will return `false` are:
//     ///
//     /// - `SqlLiteral` (We don't know if the SQL was constructed dynamically, so
//     ///   we must assume that it was)
//     /// - `In` and `NotIn` (Each value requires a separate bind param
//     ///   placeholder)
//     ///
//     /// This method should only be called by implementations of `Connection`.
//     fn is_safe_to_cache_prepared(&self) -> QueryResult<bool> {
//         let mut result = true;
//         self.walk_ast(AstPass::is_safe_to_cache_prepared(&mut result))?;
//         Ok(result)
//     }

//     #[doc(hidden)]
//     /// Does walking this AST have any effect?
//     fn is_noop(&self) -> QueryResult<bool> {
//         let mut result = true;
//         self.walk_ast(AstPass::is_noop(&mut result))?;
//         Ok(result)
//     }
// }
// pub enum Error {}
// type QueryResult<T> = Result<T, Error>;

pub use self::reproduce::AstPass;

mod reproduce {
    // use super::{QueryFragment, QueryResult, ValuesClause, AstPass};
    // use super::{QueryFragment, QueryResult, ValuesClause, Backend};
    use super::{QueryResult, ValuesClause, QueryFragment, Backend};

    pub struct AstPass<'a, DB>
    {
    }

    trait Insertable {
        type Values;

        fn values(self) -> Self::Values;
    }

    impl<T> Insertable for Option<T>
    where
        T: Insertable,
        T::Values: Default,
    {
        type Values = T::Values;

        fn values(self) -> Self::Values {
        }
    }

    impl<'a, T> Insertable for &'a Option<T>
    where
        Option<&'a T>: Insertable,
    {
        type Values = <Option<&'a T> as Insertable>::Values;

        fn values(self) -> Self::Values {
        }
    }

    pub struct BatchInsert {
    }

    impl<'a, T, DB> QueryFragment<DB> for BatchInsert
    where
        &'a T: Insertable<Values = ValuesClause<(), ()>>,
    {
        fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        }
    }

}