//TODO: Remove dead_code
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_must_use)]



#[macro_use] pub mod db;
#[cfg(feature = "auth")]
pub mod auth;
#[cfg(feature = "model")]
pub mod client;

pub mod default {
    use tokio_postgres::Config;
    use lazy_static::lazy_static;
    use crate::{
      client::{Client, ClientError},
      auth::roles::*
    };


    pub async fn unauthenticated() -> Result<Client<Unauthenticated>, ClientError> {
      Client::new(&DEFAULT_UNAUTHENTICATED_CONFIG).await
    }

    pub async fn registered() -> Result<Client<Registered>, ClientError> {
      Client::new(&DEFAULT_REGISTERED_CONFIG).await
    }

    pub async fn author() -> Result<Client<Author>, ClientError> {
      Client::new(&DEFAULT_AUTHOR_CONFIG).await
    }

    pub async fn moderator() -> Result<Client<Moderator>, ClientError> {
      Client::new(&DEFAULT_MODERATOR_CONFIG).await
    }

    pub async fn admin() -> Result<Client<Admin>, ClientError> {
      Client::new(&DEFAULT_ADMIN_CONFIG).await
    }

    pub async fn auth() -> Result<Client<Auth>, ClientError> {
      Client::new(&DEFAULT_AUTH_CONFIG).await
    }

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

        pub static ref DEFAULT_STORE_CONFIG: Config = {
            let mut config = Config::new();
            config
                .user("keter_media_store")
                .password("keter_media_store")
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
        let author = db.author().await.expect("author connection failed");
        let moderator = db.moderator().await.expect("moderator connection failed");
        let admin = db.admin().await.expect("admin connection failed");

        let auth_db = db::AuthDB::default();

        let auth = auth_db.auth().await.expect("auth connection failed");
    }

    #[tokio::test]
    async fn register_user() {
        //TODO: Determine, where hashing stage should be completed
        use keter_media_model::userinfo::{RegisterData, LoginData};
        let db = db::AuthDB::default();
        let auth = db.auth().await.expect("auth connection failed");

        auth.register_user("Second user",  
        &hex::decode("263c33a2a9431fc185a3da10a199b36aadad10c59eaf10beca8b54684171ba0c").unwrap(),
        &"seconduser@mail.com".to_owned()
        ).await.expect("Failed to register user");
    }
 
    #[tokio::test]
    async fn authenticator() {
        use keter_media_model::userinfo::{RegisterData, LoginData};
        use auth::Authenticator;
        let db = db::AuthDB::default();
        let auth = Authenticator::new(db.auth().await.expect("auth connection failed"));

        let login_d = LoginData { 
            email: "thirduser@mail.com".to_owned(), 
            password: "Third user".to_owned()
        };

        let reg_d = RegisterData {
            user_name: "Third user".to_owned(),
            login_data: login_d.clone()
        };

        auth.register(reg_d).await.expect("Failed to register third user");
        auth.authenticate(login_d).await.expect("Failed to login");
    }
}