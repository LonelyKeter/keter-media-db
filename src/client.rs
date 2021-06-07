use std::{marker::PhantomData, sync::Arc};

use crate::{
    db::{
        PostgresClient,
        establish_connection
    },
    auth::roles::*,
    queries::FromQueryRowError
};

#[derive(Clone)]
pub struct Client<R: Role> {
  pub(crate) client: Arc<PostgresClient>,
  _role: PhantomData<R>
} 

unsafe impl<R: Role> Send for Client<R> { }
unsafe impl<R: Role> Sync for Client<R> { }

impl<R: Role> Client<R> {
  pub(crate) async fn new(config: &str) -> Result<Self, tokio_postgres::Error> {

    Ok( 
      Self {
        client: Arc::new(establish_connection(config).await?),
        _role: PhantomData
      }
    )
  }
}

pub enum ClientError {
  Postgres(tokio_postgres::Error), 
  Parse(FromQueryRowError)
}

impl From<tokio_postgres::Error> for ClientError {
  fn from(other: tokio_postgres::Error) -> ClientError {
    ClientError::Postgres(other)
  }
}

impl From<FromQueryRowError> for ClientError {
  fn from(other: FromQueryRowError) -> ClientError {
    ClientError::Parse(other)
  }
}