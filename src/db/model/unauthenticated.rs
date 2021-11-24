use std::option;

#[macro_use]
use super::*;

use crate::auth::roles;

use keter_media_model::{media::*, usage::*, userinfo::*};

impl Client<roles::Unauthenticated> {
    pub async fn get_media_many(&self) -> ResultSelectMany<MediaInfo> {
        self.query(Statements::GetMediaMany, &[]).await
    }

    pub async fn get_media_many_with_options(
        &self,
        options: &MediaFilterOptions,
    ) -> ResultSelectMany<MediaInfo> {

        self.query(Statements::GetMediaFiltered, 
         &[
            &options.title, 
            &options.kinds,
            &options.popularity,
            &options.times_used
            ]).await
    }

    pub async fn get_media_id(&self, key: MediaKey) -> ResultOptional<MediaInfo> {
        self.query_opt(Statements::GetMediaId, &[&key]).await
    }

    pub async fn get_media_author_id(&self, author_id: UserKey) -> ResultSelectMany<MediaInfo> {
        self.query(Statements::GetMediaAuthorId, &[&author_id])
            .await
    }

    pub async fn get_materials(
        &self,
        media: MediaKey,
        user_id: Option<UserKey>,
    ) -> ResultSelectMany<MaterialInfo> {
        self.query(Statements::GetMaterialsMedia, &[&media, &user_id])
            .await
    }

    pub async fn get_material_id(
        &self,
        material_id: MaterialKey,
        user_id: Option<UserKey>,
    ) -> ResultOptional<MaterialInfo> {
        self.query_opt(Statements::GetMaterialWithId, &[&material_id, &user_id])
            .await
    }

    pub async fn get_user_usages(&self, user_id: UserKey) -> ResultSelectMany<Usage> {
        self.query(Statements::GetUserUsages, &[&user_id]).await
    }

    pub async fn get_media_usages(&self, media_id: MediaKey) -> ResultSelectMany<UserUsage> {
        self.query(Statements::GetMediaUsages, &[&media_id]).await
    }

    pub async fn get_material_usages(&self, media_id: MediaKey) -> ResultSelectMany<UserUsage> {
        self.query(Statements::GetMaterialUsages, &[&media_id])
            .await
    }

    pub async fn get_material_usage_user_id(
        &self,
        material_id: MaterialKey,
        user_id: UserKey,
    ) -> ResultOptional<Usage> {
        self.query_opt(
            Statements::GetMaterialUsageUserId,
            &[&material_id, &user_id],
        )
        .await
    }

    pub async fn get_material_download_name(
        &self,
        material_id: MaterialKey,
    ) -> ResultSelectOne<String> {
        self.query_val(Statements::GetMaterialDownloadName, &[&material_id])
            .await?
            .extract()
    }

    pub async fn get_user_info(&self, user_id: UserKey) -> ResultOptional<UserInfo> {
        self.query_opt(Statements::GetUserInfo, &[&user_id]).await
    }

    pub async fn get_user_priveleges(&self, user_id: UserKey) -> ResultOptional<UserPriveleges> {
        self.query_opt(Statements::GetUserPriveleges, &[&user_id])
            .await
    }

    pub async fn get_authors(&self) -> ResultSelectMany<AuthorInfo> {
        todo!()
    }

    pub async fn get_reviews(&self, search_key: &MediaSearchKey) -> ResultSelectMany<UserReview> {
        match search_key {
            MediaSearchKey::TitleAuthor { title, author } => Err(ClientError::Unimplemented),
            MediaSearchKey::Id(media_id) => {
                self.query(Statements::GetReviewsMediaId, &[&*media_id])
                    .await
            }
        }
    }

    pub async fn get_review(&self, id: ReviewKey) -> ResultOptional<UserReview> {
        self.query_opt(Statements::GetReviewId, &[&id]).await
    }

    pub async fn get_license(&self, key: &LicenseSearchKey) -> ResultOptional<License> {
        match key {
            LicenseSearchKey::Id(id) => self.query_opt(Statements::GetLicenseWithId, &[&*id]).await,
            LicenseSearchKey::Title(name) => {
                self.query_one(Statements::GetLicenseWithTitle, &[&name])
                    .await
            }
        }
    }

    pub async fn get_licenses_many(&self) -> ResultSelectMany<License> {
        self.query(Statements::GetLicensesMany, &[]).await
    }
}

use enum_map::{enum_map, Enum};
use postgres_query::client::GenericClient;
#[derive(Enum, Clone, Copy, Debug)]
pub enum Statements {
    GetMediaMany,
    GetMediaFiltered,
    GetMediaId,
    GetMediaAuthorId,
    GetMediaUsages,

    GetMaterialsMedia,
    GetMaterialWithId,
    GetMaterialUsages,
    GetMaterialUsageUserId,
    GetMaterialDownloadName,

    GetReviewsMediaId,
    GetReviewId,

    GetLicensesMany,
    GetLicenseWithId,
    GetLicenseWithTitle,

    GetUserInfo,
    GetUserPriveleges,
    GetUserUsages,
}

#[async_trait]
impl InitStatements for roles::Unauthenticated {
    type StatementKey = Statements;

    async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Statements> {
        use tokio_postgres::types::Type;

        let mut statements = enum_map! {
            Statements::GetMediaMany => client.prepare_static(include_str!("sql/unauthenticated/get_media_many.sql"))
                .await.map_err(|error| InitStatementsError {statement_key: Statements::GetMediaMany, error})?,            
            Statements::GetMediaFiltered => client.prepare_typed(
                include_str!("sql/unauthenticated/get_media_filtered.sql"),
                &[String::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetMediaFiltered, error})?,
            Statements::GetMediaId => client.prepare_typed(
                include_str!("sql/unauthenticated/get_media_id.sql"),
                &[MediaKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetMediaId, error})?,
            Statements::GetMediaAuthorId => client.prepare_typed(
                include_str!("sql/unauthenticated/get_media_with_author_id.sql"),
                &[UserKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetMediaAuthorId, error})?,
            Statements::GetMaterialsMedia => client.prepare_typed(
                include_str!("sql/unauthenticated/get_materials_media.sql"),
                &[MediaKey::SQL_TYPE, UserKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetMaterialsMedia, error})?,
            Statements::GetMaterialWithId => client.prepare_typed(
                include_str!("sql/unauthenticated/get_material_with_id.sql"),
                &[MaterialKey::SQL_TYPE, UserKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetMaterialWithId, error})?,
            Statements::GetMaterialUsageUserId => client.prepare_typed(
                include_str!("sql/unauthenticated/get_material_usage_user_id.sql"),
                &[MaterialKey::SQL_TYPE, UserKey::SQL_TYPE])
                .await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetMaterialUsageUserId, error})?,
            Statements::GetMaterialDownloadName => client.prepare_typed(
                include_str!("sql/unauthenticated/get_material_download_name.sql"),
                &[MaterialKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetMaterialDownloadName, error})?,
            Statements::GetUserUsages => client.prepare_typed(
                include_str!("sql/unauthenticated/get_user_usages.sql"),
                &[UserKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetMediaMany, error})?,
            Statements::GetReviewsMediaId => client.prepare_typed(
                include_str!("sql/unauthenticated/get_reviews_media_id.sql"),
                &[MediaKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetReviewsMediaId, error})?,
            Statements::GetReviewId => client.prepare_typed(
                include_str!("sql/unauthenticated/get_review_id.sql"),
                &[ReviewKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetReviewId, error})?,
            Statements::GetUserInfo => client.prepare_typed(
                include_str!("sql/unauthenticated/get_user_info.sql"),
                &[UserKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetUserInfo, error})?,
            Statements::GetLicensesMany => client.prepare_static(include_str!("sql/unauthenticated/get_licenses_many.sql"))
                .await.map_err(|error| InitStatementsError {statement_key: Statements::GetLicensesMany, error})?,
            Statements::GetLicenseWithId => client.prepare_typed(
                include_str!("sql/unauthenticated/get_license_with_id.sql"),
                &[LicenseKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetLicenseWithId, error})?,
            Statements::GetLicenseWithTitle => client.prepare_typed(
                include_str!("sql/unauthenticated/get_license_with_title.sql"),
                &[String::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetLicenseWithTitle, error})?,
            Statements::GetMediaUsages => client.prepare_typed(
                include_str!("sql/unauthenticated/get_media_usages.sql"),
                &[MediaKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetMediaUsages, error})?,
            Statements::GetMaterialUsages => client.prepare_typed(
                include_str!("sql/unauthenticated/get_material_usages.sql"),
                &[MaterialKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetMaterialUsages, error})?,
            Statements::GetUserPriveleges => client.prepare_typed(
                include_str!("sql/unauthenticated/get_user_priveleges.sql"),
                &[UserKey::SQL_TYPE]).await
                .map_err(|error| InitStatementsError {statement_key: Statements::GetUserPriveleges, error})?,
        };

        Ok(statements)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    use pretty_unwrap::*;

    #[tokio::test]
    async fn get_media_many() {
        let client = default::unauthenticated().await.unwrap_pretty();

        let many = client.get_media_many().await.unwrap_pretty();
        assert!(many.len() > 0);
    }

    #[tokio::test]
    async fn get_media_specific() {
        let client = default::unauthenticated().await.unwrap_pretty();

        let present = client.get_media_id(1).await.unwrap_pretty();
        assert_eq!(present.unwrap().id, 1);

        let abscent = client.get_media_id(MediaKey::MAX).await.unwrap_pretty();
        assert_eq!(abscent, None);
    }
}
