#[macro_use] use super::{*};

use crate::{
    auth::roles,
    queries::{FromQueryRow, FromQueryRowError}
    };

use keter_media_model::{
    media::*,
    userinfo::*
};

impl Client<roles::Unauthenticated> {
    pub async fn get_media_many(&self) -> ResultGetMany<MediaInfo> {
        todo!()
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

    pub async fn get_reviews(&self, search_key: MediaSearchKey) -> ResultGetMany<Review> {
        todo!()
    }
}

pub struct GetMediaOptions {
    
}

use crate::insert_statement;
#[async_trait]
impl InitStatements for roles::Unauthenticated {
    async fn init_statements(client: &PostgresClient) -> InitStatementsResult {
        let mut map = StatementCollection::new();

        insert_statement!(client, map, "unauthenticated", "get_media_many");
        insert_statement!(client, map, "unauthenticated", "get_media_filtered");
        unimplemented!();
    }
}