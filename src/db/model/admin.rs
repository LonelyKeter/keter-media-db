use super::{*};

use crate::{
    auth::roles,
    };

use keter_media_model::{
    media::*,
    userinfo::*
};
use tokio_postgres::{Statement, Row};


impl Client<roles::Admin> {
    //TODO: Remove review DB logic
}


#[async_trait]
impl InitStatements for roles::Admin {
  async fn init_statements(client: &PostgresClient) -> InitStatementsResult {
    let mut statemnets = StatementCollection::new();
        Ok(statemnets)
  }
}