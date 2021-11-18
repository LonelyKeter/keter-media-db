use super::{roles::Unauthenticated, Priveleges};

use crate::db::{
    model::{GetMediaOptions, MediaSearchKey},
    result::*,
};

use keter_media_model::{media::*, usage::*, userinfo::*};

impl Priveleges<Unauthenticated> {
    pub async fn get_media_many(&self) -> ResultSelectMany<MediaInfo> {
        self.client.get_media_many().await
    }

    pub async fn get_media_many_with_options(
        &self,
        options: &GetMediaOptions,
    ) -> ResultSelectMany<MediaInfo> {
        self.client.get_media_many_with_options(options).await
    }

    pub async fn get_media_id(&self, key: MediaKey) -> ResultOptional<MediaInfo> {
        self.client.get_media_id(key).await
    }

    pub async fn get_media_author_id(&self, author_id: UserKey) -> ResultSelectMany<MediaInfo> {
        self.client.get_media_author_id(author_id).await
    }

    pub async fn get_materials(&self, media_key: MediaKey, user_id: Option<UserKey>) -> ResultSelectMany<MaterialInfo> {
        self.client.get_materials(media_key, user_id).await
    }

    pub async fn get_material_id(&self, material_id: MaterialKey, user_id: Option<UserKey>) -> ResultOptional<MaterialInfo> {
        self.client.get_material_id(material_id, user_id).await
    }       

    pub async fn get_material_usage_user_id(
        &self,
        material_id: MaterialKey,
        user_id: UserKey,
    ) -> ResultOptional<Usage> {
        self.client.get_material_usage_user_id(material_id, user_id)
            .await
    }

    pub async fn get_user_info(&self, user_id: UserKey) -> ResultOptional<UserInfo> {
        self.client.get_user_info(user_id).await
    }

    pub async fn get_user_priveleges(&self, user_id: UserKey) -> ResultOptional<UserPriveleges> {
        self.client.get_user_priveleges(user_id).await
    }

    pub async fn get_authors(&self) -> ResultSelectMany<AuthorInfo> {
        self.client.get_authors().await
    }

    pub async fn get_tags(&self) -> ResultSelectMany<Tag> {
        self.client.get_tags().await
    }

    pub async fn get_reviews(&self, search_key: MediaSearchKey) -> ResultSelectMany<UserReview> {
        self.client.get_reviews(&search_key).await
    }

    pub async fn get_review(&self, id: ReviewKey) -> ResultOptional<UserReview> {
        self.client.get_review(id).await
    }

    pub async fn get_license(&self, search_key: LicenseSearchKey) -> ResultOptional<License> {
        self.client.get_license(&search_key).await
    }

    pub async fn get_licenses_many(&self) -> ResultSelectMany<License> {
        self.client.get_licenses_many().await
    }

    pub async fn get_user_usages(&self, user_id: UserKey) -> ResultSelectMany<Usage> {
        self.client.get_user_usages(user_id).await
    }

    pub async fn get_media_usages(&self, media_id: MediaKey) -> ResultSelectMany<UserUsage> {
        self.client.get_media_usages(media_id).await
    }

    pub async fn get_material_usages(&self, material_id: MaterialKey) -> ResultSelectMany<UserUsage> {
        self.client.get_material_usages(material_id).await
    }

    pub async fn get_material_download_name(&self, material_id: MaterialKey) -> ResultSelectOne<String> {
        self.client.get_material_download_name(material_id).await
    }
}
