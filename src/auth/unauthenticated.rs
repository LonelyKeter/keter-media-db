use super::{
    Privelegies,
    roles::Unauthenticated
};

use crate::db::{
    result::*,
    model::{
        GetMediaOptions,
        MediaSearchKey
    }
};

use keter_media_model::{
  media::*,
  userinfo::*
};

impl Privelegies<Unauthenticated> {
    pub async fn get_media_many(&self) -> ResultGetMany<MediaInfo> {
        self.client.get_media_many().await
    }

    pub async fn get_media_many_with_options(&self, options: &GetMediaOptions) -> ResultGetMany<MediaInfo> {
        self.client.get_media_many_with_options(options).await
    }
    
    pub async fn get_media_id(&self, key: MediaKey) -> ResultOptional<MediaInfo> {
        self.client.get_media_id(key).await
    }

    pub async fn get_materials(&self, media_key: MediaKey) -> ResultGetMany<MaterialInfo> {
        self.client.get_materials(media_key).await
    }

    pub async fn get_author_id(&self, id: UserKey) -> ResultOptional<AuthorInfo> {
        self.client.get_author_id(id).await
    }

    pub async fn get_authors(&self) -> ResultGetMany<AuthorInfo> {
        self.client.get_authors().await
    }

    pub async fn get_tags(&self) -> ResultGetMany<Tag> {
        self.client.get_tags().await
    }

    pub async fn get_reviews(&self, search_key: MediaSearchKey) -> ResultGetMany<ReviewInfo> {
        self.client.get_reviews(search_key).await
    }
}

