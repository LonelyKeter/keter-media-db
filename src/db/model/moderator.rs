use super::{*};

use crate::{
    auth::roles,
    };

use keter_media_model::{
    media::*,
    userinfo::*
};
use tokio_postgres::{Statement, Row};


impl Client<roles::Moderator> {
    //TODO: Remove review DB logic
    pub async fn remove_review(&self, remove_review: &RemoveReview) -> ResultInsertOne {
        todo!()
    }
    
    pub async fn get_moderation(&self, search_options: &ModerationSearchOptions) -> ResultSelectOne<AuthorContacts> {
        todo!("Moderation logic")
    }

    pub async fn get_removal_reasons(&self) -> ResultSelectMany<ReviewRemovalReason> {
        todo!()
    }
}

pub struct ModerationSearchOptions {

}

use enum_map::{Enum, enum_map};
#[derive(Enum, Clone, Copy)]
pub enum Statements {

}

#[async_trait]
impl InitStatements for roles::Moderator {
    type StatementKey = Statements;

    async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Statements> {
        let mut statements = enum_map! {

        };
    
        Ok(statements)
    }
}