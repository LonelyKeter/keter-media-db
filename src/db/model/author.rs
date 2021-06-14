use super::{*};

use crate::{
    auth::roles,
    };

use keter_media_model::{
    media::*,
    userinfo::*,
    usage::*
};
use tokio_postgres::{Statement, Row};


impl Client<roles::Author> {
    pub async fn post_media(
        &self, 
        media: MediaSearchKey, 
        review: &Review) 
        -> ResultPostOne {
        todo!()
    }

    pub async fn post_materials(
        &self, media: 
        MediaSearchKey, 
        materials: &[MaterialInfo]) 
        -> ResultPostMany<MaterialInfo> {
        todo!()
    }

    pub async fn delete_material(
        &self,
        id: MaterialKey)
        -> ResultDeleteOne {
        todo!()
    }    

    //TODO: Abstract license key 
    //TODO: Should update/insert operations require id for stability 
    pub async fn update_license_media(
        &self, 
        media: MediaSearchKey,
        license: String
    ) -> ResultUpdateOne<()> {
        todo!()
    }

    pub async fn update_license_material(
        &self,
        material: MaterialKey,
        license: String
    ) -> ResultUpdateOne<()> {
        todo!()
    }

    pub async fn get_usage(
        &self, 
        author: AuthorSearchKey
    ) -> ResultGetMany<Usage> {
        todo!()
    }
}


#[async_trait]
impl InitStatements for roles::Author {
    async fn init_statements(client: &PostgresClient) -> InitStatementsResult {
        let mut statemnets = StatementCollection::new();
        Ok(statemnets)
    }
}