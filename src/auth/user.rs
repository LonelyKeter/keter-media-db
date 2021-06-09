use super::{
    Privelegies,
    roles::User
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

impl Privelegies<User> {
    pub async fn post_review(&self, search_key: &MediaSearchKey, review: &Review) -> ResultPostOne {
        self.client.post_review(self.user_id.unwrap().0, search_key, review).await
    }

    pub async fn get_author_contacts_named(&self, author: String) -> ResultGetOne<Option<AuthorContacts>> {
        self.client.get_author_contacts_named(author).await
    }

    pub async fn get_info(&self) -> ResultGetOne<Option<UserInfo>> {
        self.client.get_info(self.user_id.unwrap().0).await
    }

    pub async fn get_privelegies(&self) -> ResultGetOne<Option<UserPrivelegies>> {
        self.client.get_privelegies(self.user_id.unwrap().0).await
    }
}

