use super::*;

use crate::{
  auth::{self, roles},
  insert_statement
};

pub struct AuthDBBBuilder {
  auth_db: AuthDB
}

impl AuthDBBBuilder {
  fn new() -> Self { Self { auth_db: AuthDB::default() } }

  pub fn with_auth(&mut self, cfg: &str) -> &mut Self {
    self.auth_db.auth = cfg.to_owned();
    self
  }

  pub fn build(self) -> AuthDB { self.auth_db }
}
pub struct AuthDB {
  auth: String
}

impl AuthDB {
  pub fn builder() -> AuthDBBBuilder {
    AuthDBBBuilder::new()
  }

  fn default() -> Self { Self { auth: String::default() } }

  pub async fn auth(&self) -> Result<Client<roles::Auth>, tokio_postgres::Error> {
    Client::new(&self.auth).await
  }
}

use keter_media_model::userinfo::*;
impl Client<roles::Auth> {
  pub async fn register_user(&self, info: RegisterData) -> ResultPostOne {
    unimplemented!()
  }

  pub async fn get_user_key_password(&self, login_key: &str) -> ResultGetOne<Option<auth::IdPassword>> {
    unimplemented!()
  }

  pub async fn has_author_permission(&self, user_key: UserKey) -> ResultGetOne<Option<bool>> {
    unimplemented!()
  }

  pub async fn has_moderator_permission(&self, user_key: UserKey) -> ResultGetOne<Option<bool>> {
    unimplemented!()
  }

  pub async fn has_admin_permission(&self, user_key: UserKey) -> ResultGetOne<Option<bool>> {
    unimplemented!()
  }
}

#[async_trait]
impl InitStatements for roles::Auth {
  async fn init_statements(client: &PostgresClient) -> InitStatementsResult {
    unimplemented!();
  }
}