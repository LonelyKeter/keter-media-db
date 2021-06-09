use tokio_postgres::Row;

pub trait FromQueryRow 
    where Self: Sized {
    fn from_query_row(row: &Row) -> Result<Self, FromQueryRowError>; 
}
#[derive(Debug)]
pub enum FromQueryRowError {

}