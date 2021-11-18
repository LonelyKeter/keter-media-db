use super::{
    Priveleges,
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

impl Priveleges<Moderator> {
    pub async fn remove_review(&self, remove_review: &RemoveReview) -> ResultInsertOne {
        self.client.remove_review(remove_review).await
    }
    
    pub async fn get_moderation(&self, search_options: &ModerationSearchOptions) -> ResultSelectOne<AuthorContacts> {
        self.client.get_moderation(search_options).await
    }

    pub async fn get_removal_reasons(&self) -> ResultSelectMany<ReviewRemovalReason> {
        self.client.get_removal_reasons().await
    }
}

