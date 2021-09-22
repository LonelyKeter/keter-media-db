use enum_map::enum_map;
use postgres_types::Type;

use super::*;

use crate::auth::roles;

use keter_media_model::{
    media::*,
    usage::{License, LicenseKey, UserUsage},
    userinfo::*,
};

impl Client<roles::Registered> {
    pub async fn post_review(
        &self,
        user_id: UserKey,
        search_key: &MediaSearchKey,
        review: &Review,
    ) -> ResultPostOne {
        match search_key {
            MediaSearchKey::Id(media_id) => {
                self.execute(
                    Statements::PostReviewWithId,
                    &[&user_id, &media_id, &review.rating, &review.text],
                )
                .await?
            }
            MediaSearchKey::TitleAuthor { title, author } => {
                return Err(ClientError::Unimplemented);
                /*
                self.execute(
                    Statements::PostReviewWithTitleAuthor,
                    &[&user_id, &title, &author, &review.rating, &review.text],
                )
                .await?
                 */
            }
        };

        Ok(())
    }

    pub async fn get_author_contacts_named(
        &self,
        author: String,
    ) -> ResultGetOne<Option<AuthorContacts>> {
        todo!()
    }

    pub async fn get_info(&self, user_id: UserKey) -> ResultOptional<UserInfo> {
        self.query_opt::<UserInfo>(Statements::GetInfo, &[&user_id])
            .await
    }

    pub async fn get_privelegies(&self, user_id: UserKey) -> ResultOptional<UserPriveleges> {
        self.query_opt::<UserPriveleges>(Statements::GetPrivelegies, &[&user_id])
            .await
    }

    pub async fn get_usages(&self, user_id: UserKey) -> ResultGetMany<UserUsage> {
        self.query(Statements::GetUsages, &[&user_id]).await
    }

    pub async fn is_material_used(
        &self,
        material_id: MaterialKey,
        user_id: UserKey,
    ) -> ResultGetOne<bool> {
        self.query_bool(Statements::GetUsages, &[&material_id, &user_id])
            .await
    }
}

use enum_map::Enum;
#[derive(Enum, Clone, Copy)]
pub enum Statements {
    GetInfo,
    GetPrivelegies,
    PostReviewWithId,
    IsMaterialUsed,
    GetUsages,
}

#[async_trait]
impl InitStatements for roles::Registered {
    type StatementKey = Statements;

    async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Statements> {
        let mut statements = enum_map! {
            Statements::GetInfo => client.prepare_typed(
                include_str!("sql\\registered\\get_info.sql"),
                &[UserKey::SQL_TYPE]).await?,
            Statements::GetPrivelegies => client.prepare_typed(
                include_str!("sql\\registered\\get_privelegies.sql"),
                &[UserKey::SQL_TYPE]).await?,
            Statements::PostReviewWithId => client.prepare_typed(
                include_str!("sql\\registered\\post_review_with_id.sql"),
                &[UserKey::SQL_TYPE, MediaKey::SQL_TYPE, Type::INT2]).await?,
            Statements::GetUsages => client.prepare_typed(
                include_str!("sql\\registered\\get_usages.sql"),
                &[UserKey::SQL_TYPE]).await?,
            Statements::IsMaterialUsed => client.prepare_typed(
                include_str!("sql\\registered\\is_material_used.sql"),
                &[MaterialKey::SQL_TYPE, UserKey::SQL_TYPE]).await?,
        };

        Ok(statements)
    }
}

#[cfg(test)]
mod test {
    use crate::{auth::roles::Registered, client::Client, db::ModelDB};

    async fn registered() -> Client<Registered> {
        Client::new(&crate::default::DEFAULT_REGISTERED_CONFIG)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn get_info_test() {
        let client = registered().await;
        let info = client.get_info(1).await.unwrap().unwrap();

        assert_eq!(info.id, 1);
        assert_eq!(info.name, "First author");
    }

    #[tokio::test]
    async fn get_privelegies() {
        let client = registered().await;
        let privelegies = client.get_privelegies(1).await.unwrap().unwrap();

        assert_eq!(privelegies.author, true);
        assert_eq!(privelegies.moderator, false);
        assert_eq!(privelegies.admin, false);
    }
}
