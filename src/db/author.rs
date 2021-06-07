use super::{*};

use crate::{
    auth::roles::Unauthenticated,
    queries::{FromQueryRow, FromQueryRowError}
    };

use keter_media_model::{
    media::*,
    userinfo::*,
    usage::*
};
use tokio_postgres::{Statement, Row};


impl Client<Author> {
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
    ) -> ResultUpdate {
        todo!()
    }

    pub async fn update_license_material(
        &self,
        material: MaterialKey,
        license: String
    ) -> ResultUpdate {
        todo!()
    }

    pub async fn get_usage(
        &self, 
        author: AuthorSearchKey
    ) -> ResultGetMany<Usage> {
        todo!()
    }
}