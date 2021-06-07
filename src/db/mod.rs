mod unauthenticated;
mod user;
mod author;
mod moderator;
mod admin;

pub use unauthenticated::*;
pub use user::*;
pub use author::*;
pub use moderator::*;
pub use admin::*;

use crate::{
  auth::roles::*,
  client::*
};

pub struct ModelDB {
  unauthenticated: String,
  user: String,
  author: String,
  moderator: String,
  admin: String,
}

impl ModelDB {
  pub async fn unauthenticated(&self) -> Result<Client<Unauthenticated>, tokio_postgres::Error> {
    Client::new(&self.unauthenticated).await
  }

  pub async fn user(&self) -> Result<Client<User>, tokio_postgres::Error> {
    Client::new(&self.user).await
  }

  pub async fn author(&self) -> Result<Client<Author>, tokio_postgres::Error> {
    Client::new(&self.author).await
  }

  pub async fn moderator(&self) -> Result<Client<Moderator>, tokio_postgres::Error> {
    Client::new(&self.moderator).await
  }

  pub async fn admin(&self) -> Result<Client<Admin>, tokio_postgres::Error> {
    Client::new(&self.admin).await
  }
}

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
async fn establish_connection(config: &str) -> Result<PostgresClient, tokio_postgres::Error> {
  use tokio_postgres::{connect, NoTls};

  let (client, connection) = connect(config, NoTls).await?;
  tokio::spawn(async move {
    connection.await;
  });

  Ok(client)
}

type ResultGetOne<T> = Result<T, ClientError>;
type ResultGetMany<T> = Result<Vec<T>, ClientError>;

type ResultPostOne = Result<(), ClientError>;
type ResultPostMany<T> = Result<Vec<T>, ClientError>;

type ResultDeleteOne = Result<(), ClientError>;

pub enum MediaSearchKey {
  Key(keter_media_model::media::MediaKey),
  TitleAuthor {title: String, author: String},
}

pub struct AuthorSearchKey {
  pub name: String
}