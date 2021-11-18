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
    ) -> ResultInsertOne {
        match search_key {
            MediaSearchKey::Id(media_id) => {
                self.execute(
                    Statements::PostReviewWithId,
                    &[&user_id, &*media_id, &review.text],
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
        author: &str,
    ) -> ResultSelectOne<Option<AuthorContacts>> {
        Err(ClientError::Unimplemented)
    }

    pub async fn create_material_usage(&self, user_id: UserKey, material_id: MaterialKey) -> ResultInsertOne {
        self.execute(Statements::CreateMaterialUsage, &[&user_id, &material_id]).await?;
        Ok(())
    }

    pub async fn insert_material_rating(&self, material_id: MaterialKey, user_id: UserKey, rating: &UserRating) -> ResultUpdateOne<()> {
        self.update_one(Statements::UpdateMaterialRating, &[&material_id, &user_id, &rating.rating]).await
    }
}

use enum_map::Enum;
#[derive(Enum, Clone, Copy)]
pub enum Statements {
    PostReviewWithId,
    CreateMaterialUsage,
    UpdateMaterialRating
}

#[async_trait]
impl InitStatements for roles::Registered {
    type StatementKey = Statements;

    async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Statements> {
        let mut statements = enum_map! {
            Statements::PostReviewWithId => client.prepare_typed(
                include_str!("sql/registered/post_review_with_id.sql"),
                &[UserKey::SQL_TYPE, MediaKey::SQL_TYPE, String::SQL_TYPE]).await?,
            Statements::CreateMaterialUsage => client.prepare_typed(
                include_str!("sql/registered/create_material_usage.sql"),
                &[UserKey::SQL_TYPE, MaterialKey::SQL_TYPE]).await?,
            Statements::UpdateMaterialRating => client.prepare_typed(
                include_str!("sql/registered/update_material_rating.sql"),
                &[MaterialKey::SQL_TYPE, UserKey::SQL_TYPE, i16::SQL_TYPE]).await?,
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
            .expect("Client creation failed")
    }
}
