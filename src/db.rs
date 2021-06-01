pub struct ModelsDB {
  
}

impl GetConnection<Admin> for ModelsDB {
  fn get_connection(&self) -> Connection<Admin> {

  } 
}

pub trait GetConnection<R: Role> {
  fn get_connection(&self) -> Connection<R> {
    
  }
}