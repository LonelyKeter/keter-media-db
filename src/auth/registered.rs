use super::{roles::Registered, Privelegies};

use crate::db::{model::MediaSearchKey, result::*};

use keter_media_model::{media::*, usage::*, userinfo::*};

impl Privelegies<Registered> {
    pub async fn post_review(
        &self,
        search_key: &MediaSearchKey,
        review: &Review,
    ) -> ResultInsertOne {
        self.client
            .post_review(self.user_key, search_key, review)
            .await
    }

    pub async fn get_author_contacts_named(
        &self,
        author: String,
    ) -> ResultSelectOne<Option<AuthorContacts>> {
        self.client.get_author_contacts_named(author).await
    }

    pub async fn get_info(&self) -> ResultOptional<UserInfo> {
        self.client.get_info(self.user_key).await
    }

    pub async fn get_privelegies(&self) -> ResultOptional<UserPriveleges> {
        self.client.get_privelegies(self.user_key).await
    }

    pub async fn is_material_used(&self, material_id: MaterialKey) -> ResultSelectOne<bool> {
        self.client
            .is_material_used(material_id, self.user_key)
            .await
    }

    pub async fn use_material(&self, material_id: MaterialKey) -> ResultInsertOne {
        self.client.use_material(self.user_key, material_id).await
    }
}
