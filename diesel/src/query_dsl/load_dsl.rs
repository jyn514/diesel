use super::RunQueryDsl;
use backend::Backend;
use connection::Connection;
use deserialize::Queryable;
use query_builder::{AsQuery, QueryFragment, QueryId};
use result::QueryResult;
use sql_types::HasSqlType;

/// The `load` method
///
/// This trait should not be relied on directly by most apps. Its behavior is
/// provided by [`RunQueryDsl`]. However, you may need a where clause on this trait
/// to call `load` from generic code.
///
/// [`RunQueryDsl`]: ../trait.RunQueryDsl.html
pub trait LoadQuery<Conn, U>: RunQueryDsl<Conn> {
    /// Load this query
    fn internal_load(self, conn: &Conn) -> QueryResult;
}

impl<Conn, T, U> LoadQuery<Conn, U> for T
where
    Conn: Connection,
    Conn::Backend: HasSqlType<T::SqlType>,
    T: AsQuery + RunQueryDsl<Conn>,
    T::Query: QueryFragment + QueryId,
    U: Queryable<T::SqlType, Conn::Backend>,
{
    fn internal_load(self, conn: &Conn) -> QueryResult {
        conn.query_by_index(self)
    }
}

/// The `execute` method
///
/// This trait should not be relied on directly by most apps. Its behavior is
/// provided by [`RunQueryDsl`]. However, you may need a where clause on this trait
/// to call `execute` from generic code.
///
/// [`RunQueryDsl`]: ../trait.RunQueryDsl.html
pub trait ExecuteDsl<Conn: Connection<Backend = DB>, DB: Backend = <Conn as Connection>::Backend>:
    Sized
{
    /// Execute this command
    fn execute(query: Self, conn: &Conn) -> QueryResult;
}

impl<Conn, DB, T> ExecuteDsl<Conn, DB> for T
where
    Conn: Connection<Backend = DB>,
    DB: Backend,
    T: QueryFragment + QueryId,
{
    fn execute(query: Self, conn: &Conn) -> QueryResult {
        conn.execute_returning_count(&query)
    }
}
