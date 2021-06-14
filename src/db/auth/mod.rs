use super::*;

use crate::{
  auth::{self, roles},
};

use postgres_query::FromSqlRow;
use tokio_postgres::Config;
pub struct AuthDBBBuilder {
  auth_db: AuthDB
}

impl AuthDBBBuilder {
  fn new() -> Self { Self { auth_db: AuthDB::default() } }

  pub fn with_auth(&mut self, cfg: &Config) -> &mut Self {
    self.auth_db.auth = Some(cfg.clone());
    self
  }

  pub fn build(self) -> AuthDB { self.auth_db }
}
pub struct AuthDB {
  auth: Option<Config>
}

impl Default for AuthDB {
  fn default() -> Self {
      use crate::default::*;

      Self {
        auth: Some(DEFAULT_AUTH_CONFIG.clone())
      }
  }
}

impl AuthDB {
  pub fn builder() -> AuthDBBBuilder {
    AuthDBBBuilder::new()
  }

  fn default() -> Self { Self { auth: None } }

  pub async fn auth(&self) -> Result<Client<roles::Auth>, ClientError> {
    if let Some(cfg) = &self.auth {
      Client::new(cfg).await
    } else {
      Err(ClientError::NoConfig)
    }
  }
}

use keter_media_model::userinfo::*;
impl Client<roles::Auth> {
  pub async fn register_user(&self, login: &str, password: &[u8], email: &str) -> ResultPostOne {
    let statement = self.statements().get(statements::REGISTER_USER).unwrap();

    let result = self.client().query_opt(
      statement, 
      &[&login, &password, &email]).await?;
    
    if let Some(_) = result {
        Ok(())
    } else {
      Err(ClientError::NoValue)
    }
  }

  pub async fn get_user_key_password(&self, email: &str) -> ResultGetOne<Option<UserIdPassHash>> {
    use postgres_query::extract::FromSqlRow;
    let statement = self.statements().get(statements::GET_USER_KEY_PASSWORD).unwrap();

    let result = self.client().query_opt(
      statement, 
      &[&email]).await?;    

    if let Some(row) = result {
      let id_password = UserIdPassHash::from_row(&row)?;
    
      Ok(Some(id_password))
    } else {
      Err(ClientError::NoValue)
    }    
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

mod statements {
  pub const REGISTER_USER: &str = "register_user";
  pub const GET_USER_KEY_PASSWORD: &str = "get_user_key_password";
}

#[async_trait]
impl InitStatements for roles::Auth {
  async fn init_statements(client: &PostgresClient) -> InitStatementsResult {
    use tokio_postgres::types::Type;
    let mut statemnets = StatementCollection::new();

    statemnets.insert(
      "register_user", 
      client.prepare(include_str!("sql\\register_user.sql")).await?
      );

    statemnets.insert(
      "get_user_key_password", 
      client.prepare(include_str!("sql\\get_user_key_password.sql")).await?
      );

    
    Ok(statemnets)
  }
}