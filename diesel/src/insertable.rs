// use result::QueryResult;

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

pub struct ValuesClause{}
pub use self::reproduce::{AstPass, QueryResult};

    // pub struct AstPass<'a, DB>
    // {
    // }

pub trait QueryFragment<DB> {
    fn walk_ast(&self, pass: AstPass<DB>) -> QueryResult<()>;
}


mod reproduce {
    use super::{QueryFragment};

    pub type QueryResult<T> = Result<T, ()>;
// pub trait QueryFragment<DB> {
//     fn walk_ast(&self, pass: AstPass<DB>) -> QueryResult<()>;
// }


    pub struct AstPass<DB>
    {
    }

    trait Insertable {
        type Values;
    }

    impl<T> Insertable for Option<T>
    where
        T: Insertable,
        T::Values: Default,
    {
        type Values = T::Values;
    }

    impl<'a, T> Insertable for &'a Option<T>
    where
        Option<&'a T>: Insertable,
    {
        type Values = <Option<&'a T> as Insertable>::Values;
    }

    pub struct BatchInsert {}

    impl<'a, T> QueryFragment<()> for BatchInsert
    where
        &'a T: Insertable<Values = ()>,
        // &'a T: Insertable,
    {
        fn walk_ast(&self, mut out: AstPass<()>) -> QueryResult<()> {
        }
    }

}