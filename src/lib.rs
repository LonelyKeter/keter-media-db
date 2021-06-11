//TODO: Remove dead_code
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use] pub mod db;
pub mod auth;
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