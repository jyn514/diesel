use backend::Backend;
use expression::{Expression, SelectableExpression};
use query_builder::*;
use query_source::QuerySource;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct DefaultSelectClause;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct SelectClause<T>(pub T);

pub trait SelectClauseExpression<QS> {
    type SelectClauseSqlType;
}

impl<T, QS> SelectClauseExpression<QS> for SelectClause<T>
where
    T: SelectableExpression<QS>,
{
    type SelectClauseSqlType = T::SqlType;
}

impl<QS> SelectClauseExpression<QS> for DefaultSelectClause
where
    QS: QuerySource,
{
    type SelectClauseSqlType = <QS::DefaultSelection as Expression>::SqlType;
}

pub trait SelectClauseQueryFragment<QS> {
    fn walk_ast(&self, source: &QS, pass: AstPass) -> QueryResult;
}

impl<T, QS, DB> SelectClauseQueryFragment<QS> for SelectClause<T>
where
    DB: Backend,
    T: QueryFragment,
{
    fn walk_ast(&self, _: &QS, pass: AstPass) -> QueryResult {
        self.0.walk_ast(pass)
    }
}

impl<QS, DB> SelectClauseQueryFragment<QS> for DefaultSelectClause
where
    DB: Backend,
    QS: QuerySource,
    QS::DefaultSelection: QueryFragment,
{
    fn walk_ast(&self, source: &QS, pass: AstPass) -> QueryResult {
        source.default_selection().walk_ast(pass)
    }
}
