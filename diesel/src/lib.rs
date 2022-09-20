
#[cfg(feature = "postgres")]
#[macro_use]
extern crate bitflags;
extern crate byteorder;
#[macro_use]
extern crate diesel_derives;
#[doc(hidden)]
pub use diesel_derives::*;

#[macro_use]
mod macros;

#[cfg(test)]
#[macro_use]
extern crate cfg_if;

#[cfg(test)]
pub mod test_helpers;

pub mod associations;
pub mod backend;
pub mod connection;
pub mod data_types;
pub mod deserialize;
#[macro_use]
pub mod expression;
pub mod expression_methods;
// // #[doc(hidden)]
pub mod insertable;
pub use self::type_impls::InsertValues;
pub mod query_builder;
pub mod query_dsl;
pub mod query_source;
#[cfg(feature = "r2d2")]
pub mod r2d2;
pub mod result;
pub mod serialize;
#[macro_use]
pub mod sql_types;
pub mod migration;
pub mod row;
pub mod types;

#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "postgres")]
pub mod pg;
#[cfg(feature = "sqlite")]
pub mod sqlite;

mod type_impls;
mod util;

pub mod dsl {
    //! Includes various helper types and bare functions which are named too
    //! generically to be included in prelude, but are often used when using Diesel.

    #[doc(inline)]
    pub use helper_types::*;

    #[doc(inline)]
    pub use expression::dsl::*;

    #[doc(inline)]
    pub use query_builder::functions::{
        delete, insert_into, insert_or_ignore_into, replace_into, select, sql_query, update,
    };
}

pub mod helper_types {
    //! Provide helper types for concisely writing the return type of functions.
    //! As with iterators, it is unfortunately difficult to return a partially
    //! constructed query without exposing the exact implementation of the
    //! function. Without higher kinded types, these various DSLs can't be
    //! combined into a single trait for boxing purposes.
    //!
    //! All types here are in the form `<FirstType as
    //! DslName<OtherTypes>>::Output`. So the return type of
    //! `users.filter(first_name.eq("John")).order(last_name.asc()).limit(10)` would
    //! be `Limit<Order<FindBy<users, first_name, &str>, Asc<last_name>>>`
    use super::query_builder::locking_clause as lock;
    use super::query_dsl::methods::*;
    use super::query_dsl::*;
    use super::query_source::joins;

    #[doc(inline)]
    pub use expression::helper_types::*;

    /// Represents the return type of `.select(selection)`
    pub type Select<Source, Selection> = <Source as SelectDsl<Selection>>::Output;

    /// Represents the return type of `.filter(predicate)`
    pub type Filter<Source, Predicate> = <Source as FilterDsl<Predicate>>::Output;

    /// Represents the return type of `.filter(lhs.eq(rhs))`
    pub type FindBy<Source, Column, Value> = Filter<Source, Eq<Column, Value>>;

    /// Represents the return type of `.for_update()`
    #[cfg(feature = "with-deprecated")]
    #[allow(deprecated)]
    pub type ForUpdate<Source> = <Source as ForUpdateDsl>::Output;

    /// Represents the return type of `.for_update()`
    #[cfg(not(feature = "with-deprecated"))]
    pub type ForUpdate<Source> = <Source as LockingDsl<lock::ForUpdate>>::Output;

    /// Represents the return type of `.for_no_key_update()`
    pub type ForNoKeyUpdate<Source> = <Source as LockingDsl<lock::ForNoKeyUpdate>>::Output;

    /// Represents the return type of `.for_share()`
    pub type ForShare<Source> = <Source as LockingDsl<lock::ForShare>>::Output;

    /// Represents the return type of `.for_key_share()`
    pub type ForKeyShare<Source> = <Source as LockingDsl<lock::ForKeyShare>>::Output;

    /// Represents the return type of `.skip_locked()`
    pub type SkipLocked<Source> = <Source as ModifyLockDsl<lock::SkipLocked>>::Output;

    /// Represents the return type of `.no_wait()`
    pub type NoWait<Source> = <Source as ModifyLockDsl<lock::NoWait>>::Output;

    /// Represents the return type of `.find(pk)`
    pub type Find<Source, PK> = <Source as FindDsl<PK>>::Output;

    /// Represents the return type of `.or_filter(predicate)`
    pub type OrFilter<Source, Predicate> = <Source as OrFilterDsl<Predicate>>::Output;

    /// Represents the return type of `.order(ordering)`
    pub type Order<Source, Ordering> = <Source as OrderDsl<Ordering>>::Output;

    /// Represents the return type of `.then_order_by(ordering)`
    pub type ThenOrderBy<Source, Ordering> = <Source as ThenOrderDsl<Ordering>>::Output;

    /// Represents the return type of `.limit()`
    pub type Limit<Source> = <Source as LimitDsl>::Output;

    /// Represents the return type of `.offset()`
    pub type Offset<Source> = <Source as OffsetDsl>::Output;

    /// Represents the return type of `.inner_join(rhs)`
    pub type InnerJoin<Source, Rhs> =
        <Source as JoinWithImplicitOnClause<Rhs, joins::Inner>>::Output;

    /// Represents the return type of `.left_join(rhs)`
    pub type LeftJoin<Source, Rhs> =
        <Source as JoinWithImplicitOnClause<Rhs, joins::LeftOuter>>::Output;

    use super::associations::HasTable;
    use super::query_builder::{AsChangeset, IntoUpdateTarget, UpdateStatement};
    /// Represents the return type of `update(lhs).set(rhs)`
    pub type Update<Target, Changes> = UpdateStatement<
        <Target as HasTable>::Table,
        <Target as IntoUpdateTarget>::WhereClause,
        <Changes as AsChangeset>::Changeset,
    >;

    /// Represents the return type of `.into_boxed::<'a, DB>()`
    pub type IntoBoxed<'a, Source, DB> = <Source as BoxedDsl<'a, DB>>::Output;

    /// Represents the return type of `.distinct()`
    pub type Distinct<Source> = <Source as DistinctDsl>::Output;

    /// Represents the return type of `.distinct_on(expr)`
    #[cfg(feature = "postgres")]
    pub type DistinctOn<Source, Expr> = <Source as DistinctOnDsl<Expr>>::Output;

    /// Represents the return type of `.single_value()`
    pub type SingleValue<Source> = <Source as SingleValueDsl>::Output;

    /// Represents the return type of `.nullable()`
    pub type NullableSelect<Source> = <Source as SelectNullableDsl>::Output;
}

pub mod prelude {
    //! Re-exports important traits and types. Meant to be glob imported when using Diesel.
    pub use associations::{GroupedBy, Identifiable};
    pub use connection::Connection;
    #[deprecated(
        since = "1.1.0",
        note = "Explicitly `use diesel::deserialize::Queryable"
    )]
    pub use deserialize::Queryable;
    pub use expression::{
        AppearsOnTable, BoxableExpression, Expression, IntoSql, SelectableExpression,
    };
    pub use expression_methods::*;
    #[doc(inline)]
    pub use insertable::Insertable;
    #[doc(hidden)]
    pub use query_dsl::GroupByDsl;
    pub use query_dsl::{BelongingToDsl, JoinOnDsl, QueryDsl, RunQueryDsl, SaveChangesDsl};

    pub use query_source::{Column, JoinTo, QuerySource, Table};
    pub use result::{ConnectionError, ConnectionResult, OptionalExtension, QueryResult};

    #[cfg(feature = "mysql")]
    pub use mysql::MysqlConnection;
    #[cfg(feature = "postgres")]
    pub use pg::PgConnection;
    #[cfg(feature = "sqlite")]
    pub use sqlite::SqliteConnection;
}

pub use prelude::*;
#[doc(inline)]
pub use query_builder::debug_query;
#[doc(inline)]
pub use query_builder::functions::{
    delete, insert_into, insert_or_ignore_into, replace_into, select, sql_query, update,
};
pub use result::Error::NotFound;

pub(crate) mod diesel {
    pub use super::*;
}
