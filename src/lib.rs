//TODO: Remove dead_code
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]



#[macro_use] pub mod db;
pub mod auth;
pub mod client;
mod queries;

pub mod default {
    use tokio_postgres::Config;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref DEFAULT_UNAUTHENTICATED_CONFIG: Config = {
            let mut config = Config::new();
            config
                .user("keter_media_unauthenticated")
                .password("keter_media_unauthenticated")
                .dbname("ketermedia")
                .host("localhost")
                .port(5432);
    
            config
        };   
    
        pub static ref DEFAULT_REGISTERED_CONFIG: Config = {
            let mut config = Config::new();
            config
                .user("keter_media_registered")
                .password("keter_media_registered")
                .dbname("ketermedia")
                .host("localhost")
                .port(5432);
    
            config
        };   
    
        pub static ref DEFAULT_AUTHOR_CONFIG: Config = {
            let mut config = Config::new();
            config
                .user("keter_media_author")
                .password("keter_media_author")
                .dbname("ketermedia")
                .host("localhost")
                .port(5432);
    
            config
        };
    
        pub static ref DEFAULT_MODERATOR_CONFIG: Config = {
            let mut config = Config::new();
            config
                .user("keter_media_moderator")
                .password("keter_media_moderator")
                .dbname("ketermedia")
                .host("localhost")
                .port(5432);
    
            config
        };
    
        pub static ref DEFAULT_ADMIN_CONFIG: Config = {
            let mut config = Config::new();
            config
                .user("keter_media_admin")
                .password("keter_media_admin")
                .dbname("ketermedia")
                .host("localhost")
                .port(5432);
    
            config
        };
    
        pub static ref DEFAULT_AUTH_CONFIG: Config = {
            let mut config = Config::new();
            config
                .user("keter_media_auth")
                .password("keter_media_auth")
                .dbname("ketermedia")
                .host("localhost")
                .port(5432);
    
            config
        };
    }

}


#[cfg(test)]
mod tests {
    use crate::*;
    #[tokio::test]
    async fn it_works() {
        
        let db = db::ModelDB::default();

        let unathenticated = db.unauthenticated().await.expect("unautheticated connection failed");
        let user = db.registered().await.expect("user connection failed");
        let author = db.author().await.expect("user connection failed");
        let moderator = db.moderator().await.expect("user connection failed");
        let admin = db.admin().await.expect("user connection failed");

        let auth_db = db::AuthDB::default();

        let auth = auth_db.auth().await.expect("auth connection failed");
    }

    #[tokio::test]
    async fn register_user() {
        //TODO: Determine, where hashing stage should be completed
        use keter_media_model::userinfo::{RegisterData, LoginData};
        let db = db::AuthDB::default();
        let auth = db.auth().await.expect("auth connection failed");

        auth.register_user(RegisterData { 
            user_name: "Second user".to_owned(),
            login_data: LoginData { 
                email: "seconduser@mail.com".to_owned(), 
                password: "263c33a2a9431fc185a3da10a199b36aadad10c59eaf10beca8b54684171ba0c".to_owned()
            }
        }).await.expect("Failed to register user");
    }
}