use query_builder::{
    AstPass, InsertStatement, QueryFragment, ValuesClause,
};
use result::QueryResult;
// pub enum Error {}
// type QueryResult<T> = Result<T, Error>;

pub trait Insertable<T> {
    type Values;

    fn values(self) -> Self::Values;

    fn insert_into(self, table: T) -> InsertStatement<T, Self::Values>
    where
        Self: Sized,
    {
    }
}

pub trait CanInsertInSingleQuery<DB> {
    fn rows_to_insert(&self) -> Option<usize>;
}

impl<'a, T, DB> CanInsertInSingleQuery<DB> for &'a T
where
    T: ?Sized + CanInsertInSingleQuery<DB>,
{
    fn rows_to_insert(&self) -> Option<usize> {
        (*self).rows_to_insert()
    }
}

pub trait InsertValues<T, DB>: QueryFragment<DB> {
    fn column_names(&self, out: AstPass<DB>) -> QueryResult<()>;
}


impl<T, Tab> Insertable<Tab> for Option<T>
where
    T: Insertable<Tab>,
    T::Values: Default,
{
    type Values = T::Values;

    fn values(self) -> Self::Values {
    }
}

impl<'a, T, Tab> Insertable<Tab> for &'a Option<T>
where
    Option<&'a T>: Insertable<Tab>,
{
    type Values = <Option<&'a T> as Insertable<Tab>>::Values;

    fn values(self) -> Self::Values {
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BatchInsert {
}

impl<'a, T, Tab, Inner, DB> QueryFragment<DB> for BatchInsert
where
    &'a T: Insertable<Tab, Values = ValuesClause<Inner, Tab>>,
{
    fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        Ok(())
    }
}