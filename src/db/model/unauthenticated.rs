use super::*;

use crate::auth::roles;

use keter_media_model::{media::*, userinfo::*};

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

    pub async fn get_media_specific(&self, key: MediaKey) -> ResultOptional<MediaInfo> {
        self.query_opt(Statements::GetMediaSpecific, &[&key]).await
    }

    pub async fn get_materials(&self, media: MediaKey) -> ResultGetMany<MaterialInfo> {
        self.query(Statements::GetMaterials, &[&media]).await
    }

    pub async fn get_authors(&self) -> ResultGetMany<AuthorInfo> {
        todo!()
    }

    pub async fn get_tags(&self) -> ResultGetMany<Tag> {
        todo!()
    }

    pub async fn get_reviews(&self, search_key: MediaSearchKey) -> ResultGetMany<ReviewInfo> {
        todo!()
    }
}


use enum_map::{enum_map, Enum};
use postgres_query::client::GenericClient;
#[derive(Enum, Clone, Copy)]
pub enum Statements {
    GetMediaMany,
    GetMediaSpecific,
    GetMaterials,
}

#[async_trait]
impl InitStatements for roles::Unauthenticated {
    type StatementKey = Statements;

    async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Statements> {
        use tokio_postgres::types::Type;

        let mut statements = enum_map! {
          Statements::GetMediaMany => client.prepare_static(include_str!("sql\\unauthenticated\\get_media_many.sql")).await?,
          Statements::GetMediaSpecific => client.prepare_typed(
                include_str!("sql\\unauthenticated\\get_media_specific.sql"),
                &[Type::INT8]).await?,
          Statements::GetMaterials => client.prepare_typed(
                include_str!("sql\\unauthenticated\\get_materials.sql"),
                &[Type::INT8]).await?,
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

        let present = client.get_media_specific(1).await.unwrap();
        assert_eq!(present.unwrap().id,  1);

        let abscent = client.get_media_specific(i64::MAX).await.unwrap();
        assert_eq!(abscent, None);
    }

    #[tokio::test]
    async fn get_materials() {
        let client = default::unauthenticated().await.unwrap();

        let materials = client.get_materials(1).await.unwrap();
        assert!(materials.len() > 0);
    }
}
