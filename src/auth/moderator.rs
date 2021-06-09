use super::{
    Privelegies,
    roles::Moderator
};

use crate::db::{
    result::*,
    model::{
        ModerationSearchOptions
    }
};

use keter_media_model::{
  media::*,
  userinfo::*
};

impl Privelegies<Moderator> {
    pub async fn remove_review(&self, remove_review: &RemoveReview) -> ResultPostOne {
        self.client.remove_review(remove_review).await
    }
    
    pub async fn get_moderation(&self, search_options: &ModerationSearchOptions) -> ResultGetOne<AuthorContacts> {
        self.client.get_moderation(search_options).await
    }

    pub async fn get_removal_reasons(&self) -> ResultGetMany<ReviewRemovalReason> {
        self.client.get_removal_reasons().await
    }
}

