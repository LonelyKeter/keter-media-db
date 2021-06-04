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



mod sql {
}