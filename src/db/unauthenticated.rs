use super::{*};

use crate::{
    auth::roles::Unauthenticated,
    queries::{FromQueryRow, FromQueryRowError}
    };

use keter_media_model::{
    media::*,
    userinfo::*
};
use tokio_postgres::{Statement, Row};


impl Client<Unauthenticated> {
    pub async fn get_media_many(&self) -> ResultGetMany<MediaInfo> {
        get_many(&self.client, self.statements["get_media_many"], &[]).await
    }

    pub async fn get_media_many_with_options(&self, options: &GetMediaOptions) -> ResultGetMany<MediaInfo> {
        todo!("Dynamic filters on queries")
    }

    pub async fn get_media(&self, key: MediaKey) -> ResultGetOne<Media> {
        todo!("Query execution param abstraction")
    }

    pub async fn get_authors(&self) -> ResultGetMany<AuthorInfo> {
        todo!()
    }

    pub async fn get_tags(&self) -> ResultGetMany<Tag> {
        todo!()
    }

    pub async fn get_reviews(&self, searh_key: MediaSearchKey) -> ResultGetMany<Review> {
        todo!()
    }
}

use keter_media_model::media::MediaKind;
pub struct GetMediaOptions {
    
}