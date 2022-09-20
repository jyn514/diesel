use query_builder::{
    // AstPass, InsertStatement, QueryFragment, ValuesClause,
};
use result::QueryResult;

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

pub struct NoReturningClause;
pub struct InsertStatement<T, U, Op = (), Ret = ()> {
    operator: Op,
    target: T,
    records: U,
    returning: Ret,
}
// pub enum Error {}
// type QueryResult<T> = Result<T, Error>;

pub struct ValuesClause{}
pub use self::reproduce::{AstPass};

pub trait QueryFragment<DB> {
    fn walk_ast(&self, pass: AstPass<DB>) -> QueryResult<()>;
}


mod reproduce {
    // use super::{QueryFragment, QueryResult, ValuesClause, AstPass};
    // use super::{QueryFragment, QueryResult, ValuesClause, Backend};
    use super::{QueryResult, QueryFragment, Backend};

    // pub struct ValuesClause<T, Tab> {

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
        &'a T: Insertable<Values = ()>,
    {
        fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        }
    }

}