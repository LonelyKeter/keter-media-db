use std::{marker::PhantomData, sync::Arc};

use crate::{
    db::{
        PostgresClient,
        establish_connection
    },
    auth::roles::*
};

#[derive(Clone)]
pub struct Client<R: Role> {
  client: Arc<PostgresClient>,
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

impl Client<Unauthenticated> {
  
}