use std::{marker::PhantomData, sync::{Arc}, collections::HashMap};

use crate::{
    db::{
        PostgresClient,
        establish_connection
    },
    auth::roles::*,
};

use tokio_postgres::{Statement, Config};

pub(crate) type StatementCollection = HashMap<&'static str, Statement>;

#[derive(Clone)]
pub struct Client<R: Role> {
  client: Arc<PostgresClient>,
  statements: HashMap<&'static str, Statement>,
  _role: PhantomData<R>
}

impl<R: Role> Client<R> {
  #[inline(always)]
  pub(crate) fn client(&self) -> &PostgresClient {
    &self.client
  }

  pub(crate) fn statements(&self) -> &StatementCollection {
    &self.statements
  }
}

unsafe impl<R: Role> Send for Client<R> { }
unsafe impl<R: Role> Sync for Client<R> { }

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
  NoValue
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