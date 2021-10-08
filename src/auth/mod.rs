mod unauthenticated;
mod registered;
mod moderator;
mod author;

use crate::{client::{
    Client,
    ClientError
}, db::InitStatements};
use roles::Role;
use keter_media_model::userinfo::UserKey;

use sha2::{Sha256, Digest}; 
pub struct Authenticator {
    client: Client<roles::Auth>,
}


use keter_media_model::userinfo::{LoginData, RegisterData};
impl Authenticator {
    pub fn new(client: Client<roles::Auth>) -> Self {
        Self { client }
    }

    pub async fn authenticate(&self, info: LoginData) 
        -> Result<UserKey, AuthenticationError> {
        if let Some(id_password) = self.client.get_user_key_password(&info.email).await? {
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

    pub async fn register(&self, register: RegisterData) 
        -> Result<(), AuthenticationError> {
        //TODO: corectness checks
        Self::check_registration_correctness(&register)?;
        let password = Sha256::digest(register.login_data.password.as_bytes());
        let email = register.login_data.email;

        self.client.register_user(&register.user_name, &password, &email).await?;

        Ok(())
    }

    fn check_registration_correctness(register: &RegisterData) -> Result<(), AuthenticationError> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum AuthenticationError{
    ClientError(ClientError),
    NoUser,
    InvalidPassword,
    NameAlreadyTaken
}

impl From<ClientError> for AuthenticationError {
    fn from(other: ClientError) -> Self {
        Self::ClientError(other)
    }
}

pub struct ModelDBClients {
    unauthenticated: Client<roles::Unauthenticated>,
    user: Client<roles::Registered>,
    author: Client<roles::Author>,
    moderator: Client<roles::Moderator>,
    admin: Client<roles::Admin>,
}

use crate::db::ModelDB;
impl ModelDBClients {
    pub async fn from_model_db(model_db: &ModelDB) -> Result<Self, ClientError> {
        Ok(Self {
            unauthenticated: model_db.unauthenticated().await?,
            user: model_db.registered().await?,
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



impl<'a> Authorizator {
    pub async fn unauthenticated_privelegies(&self) 
        -> AuthorizationResult<Privelegies<roles::Unauthenticated>> {
            Ok(Privelegies::new(0, self.model_clients.unauthenticated.clone()))
    }

    pub async fn registered_privelegies(&self, user_key: UserKey) 
        -> AuthorizationResult<Privelegies<roles::Registered>> {
        Ok(Privelegies::new(user_key, self.model_clients.user.clone()))
    }

    pub async fn author_privelegies(&self, user_key: UserKey)
        -> AuthorizationResult<Privelegies<roles::Author>> {
        if self.auth_client.has_author_permission(user_key).await? {
            Ok(Privelegies::new(user_key, self.model_clients.author.clone()))
        } else {
            Err(AuthorizationError::NoAccess)
        }
    }

    pub async fn moderator_privelegies(&self, user_key: UserKey) 
        -> AuthorizationResult<Privelegies<roles::Moderator>> {
        if self.auth_client.has_moderator_permission(user_key).await? {
            Ok(Privelegies::new(user_key, self.model_clients.moderator.clone()))
        } else {
            Err(AuthorizationError::NoAccess)
        }
    }

    pub async fn admin_privelegies(&self, user_key: UserKey) 
        -> AuthorizationResult<Privelegies<roles::Admin>> {
        if self.auth_client.has_admin_permission(user_key).await? {
            Ok(Privelegies::new(user_key, self.model_clients.admin.clone()))
        } else {
            Err(AuthorizationError::NoAccess)
        }
    }    
}

use std::marker::PhantomData;
pub struct Privelegies<R: Role + InitStatements> {
    user_key: UserKey,
    client: Client<R>,
    _role: PhantomData<R>
}

impl<R: Role + InitStatements> Privelegies<R> {
    fn new(user_key: UserKey, client: Client<R>) -> Self {
        Self {
            user_key,
            client,
            _role: PhantomData
        }
    }
}

pub mod roles 
{
    pub trait Role { }
    #[cfg(feature = "model")]
    pub use model::*;
    #[cfg(feature = "auth")]
    pub use auth::*;

    #[cfg(feature = "model")]
    mod model {
        use super::Role;

        #[derive(Clone, Copy)]
        pub struct Unauthenticated;
        impl Role for Unauthenticated {}
        
        #[derive(Clone, Copy)]
        pub struct Registered;
        impl Role for Registered {}
        
        #[derive(Clone, Copy)]
        pub struct Author;
        impl Role for Author {}
        
        #[derive(Clone, Copy)]
        pub struct Moderator;
        impl Role for Moderator {}
        
        #[derive(Clone, Copy)]
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