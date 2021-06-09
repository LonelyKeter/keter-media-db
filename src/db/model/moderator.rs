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


impl Client<roles::Moderator> {
    //TODO: Remove review DB logic
    pub async fn remove_review(&self, remove_review: &RemoveReview) -> ResultPostOne {
        todo!()
    }
    
    pub async fn get_moderation(&self, search_options: &ModerationSearchOptions) -> ResultGetOne<AuthorContacts> {
        todo!("Moderation logic")
    }

    pub async fn get_removal_reasons(&self) -> ResultGetMany<ReviewRemovalReason> {
        todo!()
    }
}

pub struct ModerationSearchOptions {

}

use crate::insert_statement;
#[async_trait]
impl InitStatements for roles::Moderator {
    async fn init_statements(client: &PostgresClient) -> InitStatementsResult {
        unimplemented!();
    }
}