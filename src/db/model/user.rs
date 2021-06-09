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
    pub async fn post_review(&self, user_id: UserKey, search_key: &MediaSearchKey, review: &Review) -> ResultPostOne {
        todo!()
    }

    pub async fn get_author_contacts_named(&self, author: String) -> ResultGetOne<Option<AuthorContacts>> {
        todo!()
    }

    pub async fn get_info(&self, user_id: UserKey) -> ResultGetOne<Option<UserInfo>> {
        todo!()
    }

    pub async fn get_privelegies(&self, user_id: UserKey) -> ResultGetOne<Option<UserPrivelegies>> {
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