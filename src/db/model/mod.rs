#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


mod unauthenticated;
mod registered;
mod author;
mod moderator;
mod admin;

use keter_media_model::userinfo::UserKey;
pub use unauthenticated::*;
pub use registered::*;
pub use author::*;
pub use moderator::*;
pub use admin::*;

use super::*;
use tokio_postgres::Config;

use crate::client::ClientError;


pub struct ModelDB {
  unauthenticated: Option<Config>,
  registered: Option<Config>,
  author: Option<Config>,
  moderator: Option<Config>,
  admin: Option<Config>,
}



impl Default for ModelDB {
  fn default() -> Self {
    use crate::default::*;
      Self {
        unauthenticated: Some(DEFAULT_UNAUTHENTICATED_CONFIG.clone()),
        registered: Some(DEFAULT_REGISTERED_CONFIG.clone()),
        author: Some(DEFAULT_AUTHOR_CONFIG.clone()),
        moderator: Some(DEFAULT_MODERATOR_CONFIG.clone()),
        admin: Some(DEFAULT_ADMIN_CONFIG.clone()),
      }
  }
}

pub struct ModelDBBuilder(ModelDB);

impl ModelDBBuilder {
  pub fn with_unauthenticated(mut self, cfg: &Config) -> Self {
    self.0.unauthenticated = Some(cfg.clone());
    self
  }

  pub fn with_registered(mut self, cfg: &Config) -> Self {
    self.0.registered = Some(cfg.clone());
    self
  }

  pub fn with_author(mut self, cfg: &Config) -> Self {
    self.0.author = Some(cfg.clone());
    self
  }

  pub fn with_moderator(mut self, cfg: &Config) -> Self {
    self.0.moderator = Some(cfg.clone());
    self
  }
  
  pub fn with_admin(mut self, cfg: &Config) -> Self {
    self.0.admin = Some(cfg.clone());
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

  pub async fn unauthenticated(&self) -> Result<Client<Unauthenticated>, ClientError> {
    if let Some(cfg) = &self.unauthenticated {
      Client::new(cfg).await
    } else {
      Err(ClientError::NoConfig)
    }
  }

  pub async fn registered(&self) -> Result<Client<Registered>, ClientError> {
    if let Some(cfg) = &self.registered {
      Client::new(cfg).await
    } else {
      Err(ClientError::NoConfig)
    }
  }

  pub async fn author(&self) -> Result<Client<Author>, ClientError> {
    if let Some(cfg) = &self.author {
      Client::new(cfg).await
    } else {
      Err(ClientError::NoConfig)
    }
  }

  pub async fn moderator(&self) -> Result<Client<Moderator>, ClientError> {
    if let Some(cfg) = &self.moderator {
      Client::new(cfg).await
    } else {
      Err(ClientError::NoConfig)
    }
  }

  pub async fn admin(&self) -> Result<Client<Admin>, ClientError> {
    if let Some(cfg) = &self.admin {
      Client::new(cfg).await
    } else {
      Err(ClientError::NoConfig)
    }
  }
}

pub enum MediaSearchKey {
  Id(keter_media_model::media::MediaKey),
  TitleAuthor {title: String, author: String},
}

pub enum UserSearchKey {
  Name(String),
  Id(UserKey)
}