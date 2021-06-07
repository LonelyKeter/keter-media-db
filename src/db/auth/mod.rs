use super::*;

use crate::{
  auth::{self, roles},
  insert_statement
};

use keter_media_model::userinfo::UserKey;

//TODO: remove auth imports (maybe)
impl Client<roles::Auth> {
  pub async fn register_user(&self, info: auth::RegistrationInfo) -> ResultPostOne {
    unimplemented!()
  }

  pub async fn get_user_id_password(&self, login: &str) -> ResultGetOne<Option<auth::IdPassword>> {
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