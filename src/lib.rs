//TODO: Remove dead_code
#![allow(dead_code)]
extern crate tokio;
extern crate tokio_postgres;
extern crate keter_media_model;

#[macro_use] mod db;
mod auth;
pub mod client;
mod queries;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::db::ModelDB;
        
        let db = ModelDB::builder()
            .with_unauthenticated("")
            .with_user("")
            .with_author("")
            .with_moderator("")
            .with_admin("")
            .build();

        let unathenticated = db.unauthenticated();
        let user = db.user();
        let author = db.author();
        let moderator = db.moderator();
        let admin = db.admin();
    }
}