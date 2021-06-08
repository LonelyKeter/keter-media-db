use crate::client::{
    Client,
    ClientError
};
use roles::Role;

pub struct AuthenticationInfo {
    login: String,
    password: String
}

use sha2::{Sha256, Digest}; 
pub struct Authenticator {
    client: Client<roles::Auth>,
}

pub struct IdPassword {
    id: UserId,
    password_hash: [u8; 32]
}

//TODO: Move to model crate
pub struct RegistrationInfo {
    login: String,
    password: String,
    mail: String
}

impl Authenticator {
    pub async fn authenticate(&self, info: AuthenticationInfo) 
        -> Result<UserId, AuthenticationError> {
        if let Some(id_password) = self.client.get_user_id_password(&info.login).await? {
            let computed = Sha256::digest(info.password.as_bytes());

            if computed.as_slice().eq(&id_password.password_hash) {
                Ok(id_password.id)
            } else {
                Err(AuthenticationError::InvalidPassword)
            }            
        } else {
            Err(AuthenticationError::NoUser)
        }
    }

    pub async fn register(&self, info: RegistrationInfo) 
        -> Result<(), AuthenticationError> {
        //TODO: corectness checks
        self.client.register_user(info).await?;

        Ok(())
    }
}

pub enum AuthenticationError{
    DbError,
    NoUser,
    InvalidPassword,
    NameAlreadyTaken
}

impl From<ClientError> for AuthenticationError {
    fn from(other: ClientError) -> Self {
        Self::DbError
    }
}

pub struct ModelDBClients {
    unauthenticated: Client<roles::Unauthenticated>,
    user: Client<roles::User>,
    author: Client<roles::Author>,
    moderator: Client<roles::Moderator>,
    admin: Client<roles::Admin>,
}

use crate::db::ModelDB;
impl ModelDBClients {
    async fn from_model_db(model_db: &ModelDB) -> Result<Self, tokio_postgres::Error> {
        Ok(Self {
            unauthenticated: model_db.unauthenticated().await?,
            user: model_db.user().await?,
            author: model_db.author().await?,
            moderator: model_db.moderator().await?,
            admin: model_db.admin().await?,
        })
    }
}

pub struct Authorizator {
    auth_client: Client<roles::Auth>,
    model_clients: ModelDBClients
}

impl Authorizator {
    pub fn new(auth_client: Client<roles::Auth>, model_clients: ModelDBClients) -> Self {
        Self {
            auth_client,
            model_clients
        }
    }
}

use keter_media_model::userinfo::UserKey;
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct UserId(UserKey);

impl UserId {
    pub fn new(key: UserKey) -> Self {
        Self(key)
    }
}

impl<'a> Authorizator {
    pub async fn unauthenticated_privelegies(&self) 
        -> AuthorizationResult<Privelegies<roles::Unauthenticated>> {
            Ok(Privelegies::new(None, self.model_clients.unauthenticated.clone()))
    }

    pub async fn user_privelegies(&self, user_id: UserId) 
        -> AuthorizationResult<Privelegies<roles::User>> {
        Ok(Privelegies::new(Some(user_id), self.model_clients.user.clone()))
    }

    pub async fn author_privelegies(&self, user_id: UserId)
        -> AuthorizationResult<Privelegies<roles::Author>> {
        if let Some(true) = self.auth_client.has_author_permission(user_id.0).await? {
            Ok(Privelegies::new(Some(user_id), self.model_clients.author.clone()))
        } else {
            Err(AuthorizationError::NoAccess)
        }
    }

    pub async fn moderator_privelegies(&self, user_id: UserId) 
        -> AuthorizationResult<Privelegies<roles::Moderator>> {
        if let Some(true) = self.auth_client.has_moderator_permission(user_id.0).await? {
            Ok(Privelegies::new(Some(user_id), self.model_clients.moderator.clone()))
        } else {
            Err(AuthorizationError::NoAccess)
        }
    }

    pub async fn admin_privelegies(&self, user_id: UserId) 
        -> AuthorizationResult<Privelegies<roles::Admin>> {
        if let Some(true) = self.auth_client.has_admin_permission(user_id.0).await? {
            Ok(Privelegies::new(Some(user_id), self.model_clients.admin.clone()))
        } else {
            Err(AuthorizationError::NoAccess)
        }
    }    
}

use std::marker::PhantomData;
pub struct Privelegies<R: Role> {
    user_id: Option<UserId>,
    client: Client<R>,
    _role: PhantomData<R>
}

impl<R: Role> Privelegies<R> {
    fn new(user_id: Option<UserId>, client: Client<R>) -> Self {
        Self {
            user_id: user_id,
            client: client,
            _role: PhantomData
        }
    }
}

pub mod roles 
{
    pub trait Role { }
    pub use model::*;
    pub use auth::*;

    #[cfg(feature = "model")]
    mod model {
        use super::Role;

        #[derive(Clone)]
        pub struct Unauthenticated;
        impl Role for Unauthenticated {}
        
        #[derive(Clone)]
        pub struct User;
        impl Role for User {}
        
        #[derive(Clone)]
        pub struct Author;
        impl Role for Author {}
        
        #[derive(Clone)]
        pub struct Moderator;
        impl Role for Moderator {}
        
        #[derive(Clone)]
        pub struct Admin;
        impl Role for Admin {}
    }

    #[cfg(feature = "auth")]
    mod auth {
        use super::Role;

        pub struct Auth;
        impl Role for Auth {}
    }
}


pub type AuthorizationResult<T> = std::result::Result<T, AuthorizationError>;

#[derive(Copy, Clone, Debug)]
pub enum AuthorizationError {
    DbError,
    NoAccess
}

impl From<ClientError> for AuthorizationError {
    fn from(other: ClientError) -> Self {
        Self::DbError
    }
}