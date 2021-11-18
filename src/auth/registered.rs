use super::{roles::Registered, Priveleges};

use crate::db::{model::MediaSearchKey, result::*};

use keter_media_model::{media::*, usage::*, userinfo::*};

impl Priveleges<Registered> {
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
        author: &str,
    ) -> ResultSelectOne<Option<AuthorContacts>> {
        self.client.get_author_contacts_named(author).await
    }

    pub async fn create_material_usage(&self, material_id: MaterialKey) -> ResultInsertOne {
        self.client.create_material_usage(self.user_key, material_id).await
    }

    pub async fn insert_material_rating(&self, material_id: MaterialKey, rating: &UserRating) -> ResultUpdateOne<()> {
        self.client.insert_material_rating(material_id, self.user_key, rating).await
    }    
}
