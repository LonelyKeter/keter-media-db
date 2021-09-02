use enum_map::enum_map;

use super::{*};

use crate::{
    auth::roles,
    };

use keter_media_model::{
    media::*,
    userinfo::*
};

impl Client<roles::Registered> {
    pub async fn post_review(&self, user_id: UserKey, search_key: &MediaSearchKey, review: &Review) -> ResultPostOne {
        match search_key {
            MediaSearchKey::Key(media_id) => self.execute(
                Statements::PostReviewWithId,
                &[&user_id, &media_id, &review.rating, &review.text]).await?,
            MediaSearchKey::TitleAuthor {title, author} =>  self.execute(
                Statements::PostReviewWithTitleAuthor,
                &[&user_id, &title, &author, &review.rating, &review.text]).await?,
        };

        Ok(())
    }

    pub async fn get_author_contacts_named(&self, author: String) -> ResultGetOne<Option<AuthorContacts>> {
        todo!()
    }

    pub async fn get_info(&self, user_id: UserKey) -> ResultGetOne<Option<UserInfo>> {
        self.query_opt::<UserInfo>(Statements::GetInfo, &[&user_id]).await
    }

    pub async fn get_privelegies(&self, user_id: UserKey) -> ResultGetOne<Option<UserPriveleges>> {
        self.query_opt::<UserPriveleges>(Statements::GetPrivelegies, &[&user_id]).await
    }
}

use enum_map::Enum;
#[derive(Enum, Clone, Copy)]
pub enum Statements {
    GetInfo,
    GetPrivelegies,
    PostReviewWithId,
    PostReviewWithTitleAuthor
}

#[async_trait]
impl InitStatements for roles::Registered {
    type StatementKey = Statements;

    async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Statements> {
        let mut statements = enum_map! {
            Statements::GetInfo => client.prepare(include_str!("sql\\registered\\get_info.sql")).await?,
            Statements::GetPrivelegies => client.prepare(include_str!("sql\\registered\\get_privelegies.sql")).await?,
            Statements::PostReviewWithId => client.prepare(include_str!("sql\\registered\\post_review_with_id.sql")).await?,
            //Actual sql code
            Statements::PostReviewWithTitleAuthor => client.prepare(include_str!("sql\\registered\\post_review_with_id.sql")).await?,
        };

        Ok(statements)
    }
}

#[cfg(test)]
mod test {
    use crate::{auth::roles::Registered, client::Client, db::ModelDB};

    async fn registered() -> Client<Registered> {
        Client::new(&crate::default::DEFAULT_REGISTERED_CONFIG).await.unwrap()
    }

    #[tokio::test]
    async fn get_info_test() {
        let client = registered().await;
        let info = client.get_info(1).await
            .unwrap()
            .unwrap();

        assert_eq!(info.id , 1); 
        assert_eq!(info.name, "First author");
    }

    #[tokio::test]
    async fn get_privelegies() {
        let client = registered().await;
        let privelegies = client.get_privelegies(1).await
            .unwrap()
            .unwrap();

        assert_eq!(privelegies.author, true); 
        assert_eq!(privelegies.moderator, false); 
        assert_eq!(privelegies.admin, false); 
    }
}