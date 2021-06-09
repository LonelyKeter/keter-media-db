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

use tokio_postgres::{Statement, types::ToSql};
async fn get_many<T: FromQueryRow>(
  client: &PostgresClient, 
  statement: &Statement, params: &[&(dyn Sync + ToSql)]
)-> Result<Vec<T>, ClientError>{
  let rows = client.query(statement, params).await?;
  let result = rows_to_vec(rows.iter())?;        
  Ok(result)
}

async fn get_one<T: FromQueryRow>(
  client: &PostgresClient, 
  statement: &Statement, params: &[&(dyn Sync + ToSql)]
)-> Result<T, ClientError>{
  let row = client.query_one(statement, params).await?;
  let result = T::from_query_row(&row)?;        
  Ok(result)
}


use tokio_postgres::Row;
use crate::queries::{FromQueryRow, FromQueryRowError};
fn rows_to_vec<'a, T: FromQueryRow>(rows: impl Iterator<Item = &'a Row>) -> Result<Vec<T>, FromQueryRowError> {
  rows.
  map(T::from_query_row)
  .collect()
}


pub(crate) type PostgresClient = tokio_postgres::Client;

//TODO: Proper error logging
pub(crate) async fn establish_connection(config: &str) -> Result<PostgresClient, tokio_postgres::Error> {
  use tokio_postgres::{connect, NoTls};

  let (client, connection) = connect(config, NoTls).await?;
  tokio::spawn(async move {
    connection.await;
  });

  Ok(client)
}

type InitStatementsResult = Result<StatementCollection, tokio_postgres::Error>;
#[async_trait]
pub trait InitStatements {
  async fn init_statements(client: &PostgresClient) -> InitStatementsResult;
}

#[macro_export]
macro_rules! insert_statement {
  ($client:ident, $map:ident, $folder:literal, $key:literal) => {    
    let statement = $client
      .prepare(include_str!(concat!("sql/", $folder, "/", $key, ".sql"))).await?;
    $map.insert("get_media_many", statement);    
  }
}

pub use result::*;
pub mod result {
  use super::*;

  pub type ResultGetOne<T> = Result<T, ClientError>;
  pub type ResultGetMany<T> = Result<Vec<T>, ClientError>;
  
  pub type ResultPostOne = Result<(), ClientError>;
  pub type ResultPostMany<T> = Result<Vec<T>, ClientError>;
  
  pub type ResultDeleteOne = Result<(), ClientError>;
  
  pub type ResultUpdateOne<Ret> = Result<Ret, ClientError>;  
}