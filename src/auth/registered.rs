use super::{
    Privelegies,
    roles::Registered
};

use crate::db::{
    result::*,
    model::{
        MediaSearchKey
    }
};

use keter_media_model::{
  media::*,
  userinfo::*
};

impl Privelegies<Registered> {
    pub async fn post_review(&self, search_key: &MediaSearchKey, review: &Review) -> ResultPostOne {
        self.client.post_review(self.user_key.unwrap(), search_key, review).await
    }

    pub async fn get_author_contacts_named(&self, author: String) -> ResultGetOne<Option<AuthorContacts>> {
        self.client.get_author_contacts_named(author).await
    }

    pub async fn get_info(&self) -> ResultGetOne<Option<UserInfo>> {
        self.client.get_info(self.user_key.unwrap()).await
    }

    pub async fn get_privelegies(&self) -> ResultGetOne<Option<UserPrivelegies>> {
        self.client.get_privelegies(self.user_key.unwrap()).await
    }
}

