use backend::SupportsReturningClause;

// simple_clause!(
//     NoReturningClause,
//     ReturningClause,
//     " RETURNING ",
//     backend_bounds = SupportsReturningClause
// );
pub use crate::insertable::NoReturningClause;
// pub type ReturningClause<E> = NoReturningClause;
use std::marker::PhantomData;
pub struct ReturningClause<E> { _data: PhantomData<E> }
