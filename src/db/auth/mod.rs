use super::*;

use crate::auth::{self, roles};

use postgres_query::FromSqlRow;
use tokio_postgres::Config;

use enum_map::enum_map;

pub struct AuthDBBBuilder {
    auth_db: AuthDB,
}

impl AuthDBBBuilder {
    fn new() -> Self {
        Self {
            auth_db: AuthDB::default(),
        }
    }

    pub fn with_auth(&mut self, cfg: &Config) -> &mut Self {
        self.auth_db.auth = Some(cfg.clone());
        self
    }

    pub fn build(self) -> AuthDB {
        self.auth_db
    }
}
pub struct AuthDB {
    auth: Option<Config>,
}

impl Default for AuthDB {
    fn default() -> Self {
        use crate::default::*;

        Self {
            auth: Some(DEFAULT_AUTH_CONFIG.clone()),
        }
    }
}

impl AuthDB {
    pub fn builder() -> AuthDBBBuilder {
        AuthDBBBuilder::new()
    }

    fn default() -> Self {
        Self { auth: None }
    }

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
    pub async fn register_user(&self, login: &str, password: &[u8], email: &str) -> ResultInsertOne {
        let result = self
            .execute(Statements::RegisterUser, &[&login, &password, &email])
            .await?;

        Ok(())
    }

    pub async fn get_user_key_password(&self, email: &str) -> ResultSelectOne<Option<UserIdPassHash>> {
        use postgres_query::extract::FromSqlRow;

        let result = self
            .query_opt::<UserIdPassHash>(Statements::GetUserKeyPassword, &[&email])
            .await?;

        Ok(result)
    }

    pub async fn has_author_permission(&self, user_key: UserKey) -> ResultSelectOne<bool> {
        self.query_val(Statements::HasAuthorPermission, &[&user_key])
            .await?
            .extract()
    }

    pub async fn has_moderator_permission(&self, user_key: UserKey) -> ResultSelectOne<bool> {
        self.query_val(Statements::HasModeratorPermission, &[&user_key])
            .await?
            .extract()
    }

    pub async fn has_admin_permission(&self, user_key: UserKey) -> ResultSelectOne<bool> {
        self.query_val(Statements::HasAdminPermission, &[&user_key])
            .await?
            .extract()
    }
}

use enum_map::Enum;
#[derive(Enum, Clone)]
pub enum Statements {
    RegisterUser,
    GetUserKeyPassword,
    HasAuthorPermission,
    HasModeratorPermission,
    HasAdminPermission,
}

#[async_trait]
impl InitStatements for roles::Auth {
    type StatementKey = Statements;
    async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Statements> {
        let statements = enum_map! {
            Statements::RegisterUser => client.prepare(include_str!("sql\\register_user.sql")).await?,
            Statements::GetUserKeyPassword => client.prepare(include_str!("sql\\get_user_key_password.sql")).await?,
            Statements::HasAuthorPermission => client.prepare_typed(
                include_str!("sql\\has_author_permission.sql"),
              &[UserKey::SQL_TYPE]).await?,
            Statements::HasModeratorPermission => client.prepare_typed(
                include_str!("sql\\has_moderator_permission.sql"),
                &[UserKey::SQL_TYPE]).await?,
            Statements::HasAdminPermission => client.prepare_typed(
                include_str!("sql\\has_admin_permission.sql"),
              &[UserKey::SQL_TYPE]).await?,
        };

        Ok(statements)
    }
}
