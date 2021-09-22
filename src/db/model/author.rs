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
}

use enum_map::{Enum, enum_map};
#[derive(Enum, Clone, Copy)]
pub enum Statements {

}


#[async_trait]
impl InitStatements for roles::Author {
    type StatementKey = Statements;

    async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Statements> {
        let statements = enum_map! {

        };

        Ok(statements)
    }
}