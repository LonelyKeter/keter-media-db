use super::*;

use crate::auth::roles;

use keter_media_model::{
    media::*,
    usage::*,
    userinfo::*,
};

impl Client<roles::Unauthenticated> {
    pub async fn get_media_many(&self) -> ResultGetMany<MediaInfo> {
        self.query(Statements::GetMediaMany, &[]).await
    }

    pub async fn get_media_many_with_options(
        &self,
        options: &GetMediaOptions,
    ) -> ResultGetMany<MediaInfo> {
        todo!("Dynamic filters on queries")
    }

    pub async fn get_media_id(&self, key: MediaKey) -> ResultOptional<MediaInfo> {
        self.query_opt(Statements::GetMediaId, &[&key]).await
    }

    pub async fn get_media_author_id(&self, author_id: UserKey) -> ResultGetMany<MediaInfo> {
        self.query(Statements::GetMediaAuthorId, &[&author_id])
            .await
    }

    pub async fn get_materials(&self, media: MediaKey) -> ResultGetMany<MaterialInfo> {
        self.query(Statements::GetMaterials, &[&media]).await
    }

    pub async fn get_material_id(&self, material: MaterialKey) -> ResultOptional<MaterialInfo> {
        self.query_opt(Statements::GetMaterialWithId, &[&material])
            .await
    }

    pub async fn get_user_info(&self, user_id: UserKey) -> ResultOptional<UserInfo> {
        self.query_opt(Statements::GetUserInfo, &[&user_id]).await
    }

    pub async fn get_authors(&self) -> ResultGetMany<AuthorInfo> {
        todo!()
    }

    pub async fn get_tags(&self) -> ResultGetMany<Tag> {
        todo!()
    }

    pub async fn get_reviews(&self, search_key: MediaSearchKey) -> ResultGetMany<UserReview> {
        match search_key {
            MediaSearchKey::TitleAuthor { title, author } => Err(ClientError::Unimplemented),
            MediaSearchKey::Id(media_id) => {
                self.query(Statements::GetReviewsMediaId, &[&media_id])
                    .await
            }
        }
    }

    pub async fn get_license(&self, key: LicenseSearchKey) -> ResultOptional<License> {
        match key {
            LicenseSearchKey::Id(id) => self.query_opt(Statements::GetLicenseWithId, &[&id]).await,
            LicenseSearchKey::Title(name) => {
                self.query_one(Statements::GetLicenseWithTitle, &[&name])
                    .await
            }
        }
    } 
}

use enum_map::{enum_map, Enum};
use postgres_query::client::GenericClient;
#[derive(Enum, Clone, Copy)]
pub enum Statements {
    GetMediaMany,
    GetMediaId,
    GetMediaAuthorId,

    GetMaterials,
    GetMaterialWithId,

    GetReviewsMediaId,

    GetLicenseWithId,
    GetLicenseWithTitle,

    GetUserInfo
}

#[async_trait]
impl InitStatements for roles::Unauthenticated {
    type StatementKey = Statements;

    async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Statements> {
        use tokio_postgres::types::Type;

        let mut statements = enum_map! {
            Statements::GetMediaMany => client.prepare_static(include_str!("sql\\unauthenticated\\get_media_many.sql")).await?,
            Statements::GetMediaId => client.prepare_typed(
                include_str!("sql\\unauthenticated\\get_media_specific.sql"),
                &[MediaKey::SQL_TYPE]).await?,
            Statements::GetMediaAuthorId => client.prepare_typed(
                include_str!("sql\\unauthenticated\\get_media_with_author_id.sql"),
                &[UserKey::SQL_TYPE]).await?,
            Statements::GetMaterials => client.prepare_typed(
                include_str!("sql\\unauthenticated\\get_materials.sql"),
                &[MediaKey::SQL_TYPE]).await?,
            Statements::GetMaterialWithId => client.prepare_typed(
                include_str!("sql\\unauthenticated\\get_material_with_id.sql"),
                &[MediaKey::SQL_TYPE]).await?,
            Statements::GetReviewsMediaId => client.prepare_typed(
                include_str!("sql\\unauthenticated\\get_reviews_media_id.sql"),
                &[MediaKey::SQL_TYPE]).await?,
            Statements::GetUserInfo => client.prepare_typed(
                include_str!("sql\\unauthenticated\\get_user_info.sql"),
                &[UserKey::SQL_TYPE]).await?,
            Statements::GetLicenseWithId => client.prepare_typed(
                include_str!("sql\\unauthenticated\\get_license_with_id.sql"),
                &[Type::INT4]).await?,
            Statements::GetLicenseWithTitle => client.prepare_typed(
                include_str!("sql\\unauthenticated\\get_license_with_title.sql"),
                &[Type::VARCHAR]).await?,
        };

        Ok(statements)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[tokio::test]
    async fn get_media_many() {
        let client = default::unauthenticated().await.unwrap();

        let many = client.get_media_many().await.unwrap();
        assert!(many.len() > 0);
    }

    #[tokio::test]
    async fn get_media_specific() {
        let client = default::unauthenticated().await.unwrap();

        let present = client.get_media_id(1).await.unwrap();
        assert_eq!(present.unwrap().id, 1);

        let abscent = client.get_media_id(MediaKey::MAX).await.unwrap();
        assert_eq!(abscent, None);
    }

    #[tokio::test]
    async fn get_materials() {
        let client = default::unauthenticated().await.unwrap();

        let materials = client.get_materials(1).await.unwrap();
        assert!(materials.len() > 0);
    }
}
