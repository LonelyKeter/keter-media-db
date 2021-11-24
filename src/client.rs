use enum_map::EnumMap;
use std::{collections::HashMap, fmt::Debug, marker::PhantomData, sync::Arc};

use crate::{auth::roles::*, db::{InitStatementsError, PostgresClient, establish_connection}};

use postgres_types::{FromSql, ToSql};
use tokio_postgres::{Config, Row, Statement};

pub(crate) type StatementCollection<TKey> = EnumMap<TKey, Statement>;

//TODO: Bound statement collection to be indexed by TKey and remove specific type


pub struct Client<R: Role + InitStatements> {
    client: PostgresClient,
    statements: StatementCollection<R::StatementKey>,
    _role: PhantomData<R>,
}

use postgres_query::FromSqlRow;
impl<R: Role + InitStatements> Client<R> {
    #[inline(always)]
    pub(crate) fn client(&self) -> &PostgresClient {
        &self.client
    }

    pub(crate) fn statements(&self) -> &StatementCollection<R::StatementKey> {
        &self.statements
    }

    pub(crate) async fn query<T: FromSqlRow>(
        &self,
        statement_key: R::StatementKey,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<T>, ClientError> {
        use postgres_query::extract::FromSqlRow;
        let statement = &self.statements[statement_key];
        let result = self.client().query(statement, params).await?;
        let values = T::from_row_multi(&result)?;

        Ok(values)
    }

    pub(crate) async fn query_one<T: FromSqlRow>(
        &self,
        statement_key: R::StatementKey,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<T, ClientError> {
        use postgres_query::extract::FromSqlRow;
        let statement = &self.statements[statement_key];
        let result = self.client().query_one(statement, params).await?;
        let value = T::from_row(&result)?;

        Ok(value)
    }

    pub(crate) async fn query_opt<T: FromSqlRow>(
        &self,
        statement_key: R::StatementKey,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<T>, ClientError> {
        use postgres_query::extract::FromSqlRow;
        let statement = &self.statements[statement_key];
        let result = self.client().query_opt(statement, params).await?;

        if let Some(row) = result {
            let value = T::from_row(&row)?;

            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn execute(
        &self,
        statement_key: R::StatementKey,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, ClientError> {
        use postgres_query::extract::FromSqlRow;
        let statement = &self.statements[statement_key];
        let result = self.client().execute(statement, params).await?;

        Ok(result)
    }

    pub(crate) async fn update_one(
        &self,
        statement_key: R::StatementKey,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<(), ClientError> {
        use postgres_query::extract::FromSqlRow;
        let statement = &self.statements[statement_key];
        let modified = self.client().execute(statement, params).await?;
        
        if modified != 1 {
            Err(ClientError::InvalidUpdate)
        } else {
            Ok(())
        }
    }

    pub(crate) async fn query_val(
        &self,
        statement_key: R::StatementKey,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<QueriedVal, ClientError> {
        let statement = &self.statements[statement_key];
        let row = self.client.query_one(statement, params).await?;        

        Ok(QueriedVal::new(row))
    }
}

pub(crate) struct QueriedVal {
    row: Row
}

impl QueriedVal {
    fn new(row: Row) -> Self {
        Self { row }
    }

    pub fn extract<'a, T: FromSql<'a>>(&'a self) -> Result<T, ClientError> {
        self.row.try_get(0).map_err(ClientError::from)
    }
}

unsafe impl<R: Role + InitStatements> Send for Client<R> {}
unsafe impl<R: Role + InitStatements> Sync for Client<R> {}

use crate::db::InitStatements;
impl<R: Role + InitStatements> Client<R> {
    pub(crate) async fn new(config: &Config) -> Result<Self, ClientError> {
        let client = establish_connection(config).await?;
        let statements = R::init_statements(&client).await?;

        Ok(Self {
            client,
            statements,
            _role: PhantomData,
        })
    }
}

#[derive(Debug)]
pub enum ClientError {
    Postgres(tokio_postgres::Error),
    Parse(postgres_query::extract::Error),
    InitStatements { statement_key: String, error: tokio_postgres::Error },
    NoConfig,
    Unimplemented,
    InvalidUpdate
}

impl From<tokio_postgres::Error> for ClientError {
    fn from(other: tokio_postgres::Error) -> ClientError {
        ClientError::Postgres(other)
    }
}

impl From<postgres_query::extract::Error> for ClientError {
    fn from(other: postgres_query::extract::Error) -> ClientError {
        ClientError::Parse(other)
    }
}

impl<K: Debug> From<InitStatementsError<K>> for ClientError {
    fn from(other: InitStatementsError<K>) -> Self {
        ClientError::InitStatements {
            statement_key: format!("{:?}", other.statement_key),
            error: other.error
        }
    }
}

#[cfg(test)]
mod test {}
