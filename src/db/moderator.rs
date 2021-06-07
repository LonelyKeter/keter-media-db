use super::{*};

use crate::{
    auth::roles::Unauthenticated,
    queries::{FromQueryRow, FromQueryRowError}
    };

use keter_media_model::{
    media::*,
    userinfo::*
};
use tokio_postgres::{Statement, Row};


impl Client<Unauthenticated> {
    //TODO: Remove review DB logic
    async fn remove_review(&self, search_key: &MediaSearchKey, review: &Review) -> ResultPostOne {
        todo!()
    }
    
    async fn get_author_contacts(&self, author: String) -> ResultGetOne<AuthorContacts> {
        todo!()
    }
}