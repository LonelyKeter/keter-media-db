use std::{marker::PhantomData, sync::{Arc}, collections::HashMap};
use enum_map::EnumMap;

use crate::{
    db::{
        PostgresClient,
        establish_connection
    },
    auth::roles::*,
};

use postgres_types::ToSql;
use tokio_postgres::{Config, Statement};

pub(crate) type StatementCollection<TKey> = EnumMap<TKey, Statement>;

pub struct Client<R: Role + InitStatements> { 
  client: Arc<PostgresClient>,
  statements: StatementCollection<R::StatementKey>,
  _role: PhantomData<R>
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
  
  pub(crate) async fn query<T: FromSqlRow>(&self, statement_key: R::StatementKey, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<T>, ClientError> {
    use postgres_query::extract::FromSqlRow;
    let statement = &self.statements[statement_key];

    let result = self.client().query(
      statement, 
      params).await?;     

    let values  = T::from_row_multi(&result)?;
    Ok(values)
  }

  pub(crate) async fn query_one<T: FromSqlRow>(&self, statement_key: R::StatementKey, params: &[&(dyn ToSql + Sync)]) -> Result<T, ClientError> {
    use postgres_query::extract::FromSqlRow;
    let statement = &self.statements[statement_key];

    let result = self.client().query_one(
      statement, 
      params).await?;     

    let value  = T::from_row(&result)?;
    Ok(value)
  }

  pub(crate) async fn query_opt<T: FromSqlRow>(&self, statement_key: R::StatementKey, params: &[&(dyn ToSql + Sync)]) -> Result<Option<T>, ClientError> {
    use postgres_query::extract::FromSqlRow;
    let statement = &self.statements[statement_key];

    let result = self.client().query_opt(
      statement, 
      params).await?;    

    if let Some(row) = result {
      let value = T::from_row(&row)?;
    
      Ok(Some(value))
    } else {
      Ok(None)
    }    
  }

  pub(crate) async fn execute(&self, statement_key: R::StatementKey, params: &[&(dyn ToSql + Sync)]) -> Result<u64, ClientError> {
    use postgres_query::extract::FromSqlRow;
    let statement = &self.statements[statement_key];

    let result = self.client().execute(
      statement, 
      params).await?;     

    Ok(result)
  }
}

impl<R> Clone for Client<R> 
  where R: Role + InitStatements,
  EnumMap<<R as InitStatements>::StatementKey, Statement>: Clone
{
  fn clone(&self) -> Self {
      Self {
        client: self.client.clone(),
        statements: self.statements.clone(),
        _role: PhantomData
      }
  }
}

unsafe impl<R: Role + InitStatements> Send for Client<R> { }
unsafe impl<R: Role + InitStatements> Sync for Client<R> { }

use crate::db::InitStatements;
impl<R: Role + InitStatements> Client<R> {
  pub(crate) async fn new(config: &Config) -> Result<Self, ClientError> {
    let client = Arc::new(establish_connection(config).await?);
    let statements = R::init_statements(&client).await?;

    Ok( 
      Self {
        client,
        statements,
        _role: PhantomData
      }
    )
  }
}

#[derive(Debug)]
pub enum ClientError {
  Postgres(tokio_postgres::Error), 
  Parse(postgres_query::extract::Error),
  NoConfig,
  NoValue,
  Unimplemented
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

#[cfg(test)]
mod test {
  
}