mod unauthenticated;
mod user;
mod author;
mod moderator;
mod admin;

pub use unauthenticated::*;
pub use user::*;
pub use author::*;
pub use moderator::*;
pub use admin::*;

use super::*;

pub enum MediaSearchKey {
  Key(keter_media_model::media::MediaKey),
  TitleAuthor {title: String, author: String},
}

pub struct AuthorSearchKey {
  pub name: String
}