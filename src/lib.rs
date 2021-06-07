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
        assert_eq!(2 + 2, 4);
    }
}