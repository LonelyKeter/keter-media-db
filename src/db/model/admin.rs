use super::{*};

use crate::{
    auth::roles,
    queries::{FromQueryRow, FromQueryRowError}
    };

use keter_media_model::{
    media::*,
    userinfo::*
};
use tokio_postgres::{Statement, Row};


impl Client<roles::Admin> {
    //TODO: Remove review DB logic
}

use crate::insert_statement;
#[async_trait]
impl InitStatements for roles::Admin {
  async fn init_statements(client: &PostgresClient) -> InitStatementsResult {
    unimplemented!();
  }
}