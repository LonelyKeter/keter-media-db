use crate::model::*;


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

pub trait Role { }

pub struct Admin(u64);
impl Role for Admin {}

impl Privelegies<Admin> {
    async fn add_license(&self, license: licenses::License) {
        self.db_connection.add_license(license);
    }
}

pub struct Unauthenticated;
impl Role for Unauthenticated { }

pub struct User(u64);
impl Role for User { }

impl Privelegies<User> {
    pub fn get_media() -> MediaPoll {
        
    }
}


pub type AuthorizationResult<T> = std::result::Result<T, AuthorizationError>;

#[derive(Copy, Clone, Debug)]
pub enum AuthorizationError {
    
}