use super::{*};

use crate::{
    auth::roles,
    queries::{FromQueryRow, FromQueryRowError}
    };

use keter_media_model::{
    media::*,
    userinfo::*
};

impl Client<roles::User> {
    async fn post_review(&self, search_key: &MediaSearchKey, review: &Review) -> ResultPostOne {
        todo!()
    }

    async fn get_author_contacts(&self, author: String) -> ResultGetOne<AuthorContacts> {
        todo!()
    }
}

use crate::insert_statement;
#[async_trait]
impl InitStatements for roles::User {
    async fn init_statements(client: &PostgresClient) -> InitStatementsResult {
        unimplemented!();
    }
}