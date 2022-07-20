#[cfg(feature = "model")]
pub mod model;
#[cfg(feature = "auth")]
pub mod auth;
use std::fmt::Debug;

use keter_media_model::SqlType;

use async_trait::async_trait;

use crate::{
  auth::roles::*,
  client::*
};

pub use model::{ModelDB, ModelDBBuilder};
pub use auth::{AuthDB, AuthDBBBuilder};

pub(crate) type PostgresClient = tokio_postgres::Client;

//TODO: Proper error logging
use tokio_postgres::{Config, Statement};
pub(crate) async fn establish_connection(config: &Config) -> Result<PostgresClient, tokio_postgres::Error> {
  use tokio_postgres::{connect, NoTls};

  let (client, connection) = config.connect(NoTls).await?;
  tokio::spawn(async move {
    connection.await;
  });

  Ok(client)
}

pub(crate) type InitStatementsResult<TKey> = Result<StatementCollection<TKey>, InitStatementsError<TKey>>;
#[async_trait]
pub trait InitStatements {
  type StatementKey: enum_map::Enum<Statement> + Debug;
  async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Self::StatementKey>;
}

pub struct InitStatementsError<K> {
    pub statement_key: K,
    pub error: tokio_postgres::Error
}

#[macro_export]
macro_rules! await_statement {
    ($key:ident, $e:expr) => {
        ($e.await.map_err(|error| InitStatementsError {statement_key: $key, error}))
    }
}

pub use await_statement;

pub use result::*;
pub mod result {
  use super::*;

  pub type ResultSelectOne<T> = Result<T, ClientError>;
  pub type ResultOptional<T> = Result<Option<T>, ClientError>;
  pub type ResultSelectMany<T> = Result<Vec<T>, ClientError>;
  
  pub type ResultInsertOne = Result<(), ClientError>;
  pub type ResultInsertMany<T> = Result<Vec<T>, ClientError>;
  
  pub type ResultDeleteOne = Result<(), ClientError>;
  
  pub type ResultUpdateOne<Ret> = Result<Ret, ClientError>;  
}

pub mod domain_types {
  use postgres_types::{ToSql, FromSql};

  #[derive(Debug, ToSql, FromSql)]
  pub struct Email(pub String);
}