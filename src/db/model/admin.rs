use super::*;

use enum_map::enum_map;

use crate::auth::roles;

use keter_media_model::{media::*, userinfo::*};
use tokio_postgres::{Row, Statement};

impl Client<roles::Admin> {
    fn get_material_usage(&self, key: UserKey) {
      todo!()
    }
}

use enum_map::Enum;
#[derive(Enum, Clone, Copy, Debug)]
pub enum Statements {}

#[async_trait]
impl InitStatements for roles::Admin {
    type StatementKey = Statements;

    async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Statements> {
        let mut statements = enum_map! {};

        Ok(statements)
    }
}
