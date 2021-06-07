use tokio_postgres::Row;

pub trait FromQueryRow 
    where Self: Sized {
    fn from_query_row(row: &Row) -> Result<Self, FromQueryRowError>; 
}

pub enum FromQueryRowError {

}



mod impls {    
    use super::*;

    mod media {
        use keter_media_model::media::*;
        use super::*;

        impl FromQueryRow for MediaInfo {
            fn from_query_row(row: &Row) -> Result<Self, FromQueryRowError> {
                todo!("Impl")
            }
        }
    }
}