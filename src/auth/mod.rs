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
        self.client.register_user(info)?;

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


pub struct Authorizator {
    client: Client<roles::Auth>
}

pub struct UserId(keter_media_model::userinfo::UserKey);

impl Authorizator {
    pub async fn unauthenticated_privelegies(&self) 
        -> AuthorizationResult<Privelegies<roles::Unauthenticated>> {
            Ok(Privelegies::new())
    }

    pub async fn user_privelegies(&self, user_id: UserId) 
        -> AuthorizationResult<Privelegies<roles::User>> {
        Ok(Privelegies::new())
    }

    pub async fn author_privelegies(&self, user_id: UserId)
        -> AuthorizationResult<Privelegies<roles::Author>> {
        if let Some(true) = self.client.has_author_permission(user_id.0).await? {
            Ok(Privelegies::new())
        } else {
            Err(AuthorizationError::NoAccess)
        }
    }

    pub async fn moderator_privelegies(&self, user_id: UserId) 
        -> AuthorizationResult<Privelegies<roles::Moderator>> {
        if let Some(true) = self.client.has_moderator_permission(user_id.0).await? {
            Ok(Privelegies::new())
        } else {
            Err(AuthorizationError::NoAccess)
        }
    }

    pub async fn admin_privelegies(&self, user_id: UserId) 
        -> AuthorizationResult<Privelegies<roles::Moderator>> {
        if let Some(true) = self.client.has_admin_permission(user_id.0).await? {
            Ok(Privelegies::new())
        } else {
            Err(AuthorizationError::NoAccess)
        }
    }    
}

use std::marker::PhantomData;
pub struct Privelegies<R: Role> {
    _role: PhantomData<R>
}

impl<R: Role> Privelegies<R> {
    fn new() -> Self {
        Self {
            _role: PhantomData
        }
    }
}

impl<R: Role> From<R> for Privelegies<R> {
    fn from(role: R) -> Self {
        Self::new()
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

        pub struct Unauthenticated;
        impl Role for Unauthenticated {}
        
        pub struct User;
        impl Role for User {}
    
        pub struct Author;
        impl Role for Author {}
    
        pub struct Moderator;
        impl Role for Moderator {}
    
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