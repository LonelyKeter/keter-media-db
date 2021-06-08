#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


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

use super::*;

#[derive(Default)]
pub struct ModelDB {
  unauthenticated: String,
  user: String,
  author: String,
  moderator: String,
  admin: String,
}

pub struct ModelDBBuilder(ModelDB);

impl ModelDBBuilder {
  pub fn with_unauthenticated(mut self, cfg: &str) -> Self {
    self.0.unauthenticated = cfg.to_owned();
    self
  }

  pub fn with_user(mut self, cfg: &str) -> Self {
    self.0.user = cfg.to_owned();
    self
  }

  pub fn with_author(mut self, cfg: &str) -> Self {
    self.0.author = cfg.to_owned();
    self
  }

  pub fn with_moderator(mut self, cfg: &str) -> Self {
    self.0.moderator = cfg.to_owned();
    self
  }
  
  pub fn with_admin(mut self, cfg: &str) -> Self {
    self.0.admin = cfg.to_owned();
    self
  }

  pub fn build(self) -> ModelDB {
    self.0
  }
}

impl ModelDB {
  pub fn builder() -> ModelDBBuilder {
    ModelDBBuilder(ModelDB::default())
  }

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

pub enum MediaSearchKey {
  Key(keter_media_model::media::MediaKey),
  TitleAuthor {title: String, author: String},
}

pub struct AuthorSearchKey {
  pub name: String
}