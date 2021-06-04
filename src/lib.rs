extern crate tokio;
extern crate tokio_postgres;

mod model;
mod db;
mod auth;
pub mod client;
mod sql;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}