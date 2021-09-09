#[cfg(feature = "model")]
pub mod model;
#[cfg(feature = "auth")]
pub mod auth;

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

type InitStatementsResult<TKey> = Result<StatementCollection<TKey>, tokio_postgres::Error>;
#[async_trait]
pub trait InitStatements {
  type StatementKey: enum_map::Enum<Statement>;
  async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Self::StatementKey>;
}

pub use result::*;
pub mod result {
  use super::*;

  pub type ResultGetOne<T> = Result<T, ClientError>;
  pub type ResultOptional<T> = Result<Option<T>, ClientError>;
  pub type ResultGetMany<T> = Result<Vec<T>, ClientError>;
  
  pub type ResultPostOne = Result<(), ClientError>;
  pub type ResultPostMany<T> = Result<Vec<T>, ClientError>;
  
  pub type ResultDeleteOne = Result<(), ClientError>;
  
  pub type ResultUpdateOne<Ret> = Result<Ret, ClientError>;  
}

pub mod domain_types {
  use postgres_types::{ToSql, FromSql};

  #[derive(Debug, ToSql, FromSql)]
  pub struct Email(pub String);
}