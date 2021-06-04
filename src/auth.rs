use crate::model::*;
use roles::*;

pub struct AuthenticationInfo {

}

pub struct Authorizator {

}

impl Authorizator {
    pub fn get_privelegies<R: Role>(&self, info: &impl Into<R>) -> AuthorizationResult<Privelegies<R>> {
        let role = info.into();
        self.check(role)?;

        Ok(Privelegies::from(role))
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


impl Privelegies<Admin> {

}


impl Privelegies<User> {
    pub fn get_media() -> Vec<MediaInfo> {
        
    }
}

pub mod roles 
{
    pub trait Role { }

    pub struct Unauthenticated;
    impl Role for Unauthenticated { }
    
    pub struct User;
    impl Role for User { }

    pub struct Author;
    impl Role for Author {}

    pub struct Moderator;
    impl Role for Moderator {}

    pub struct Admin;
    impl Role for Admin {}
}


pub type AuthorizationResult<T> = std::result::Result<T, AuthorizationError>;

#[derive(Copy, Clone, Debug)]
pub enum AuthorizationError {
    
}