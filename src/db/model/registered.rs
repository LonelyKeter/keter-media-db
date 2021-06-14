use self::statements::*;

use super::{*};

use crate::{
    auth::roles,
    };

use keter_media_model::{
    media::*,
    userinfo::*
};

impl Client<roles::Registered> {
    pub async fn post_review(&self, user_id: UserKey, search_key: &MediaSearchKey, review: &Review) -> ResultPostOne {
        match search_key {
            &MediaSearchKey::Key(media_id) => self.execute(
                POST_REVIEW_WITH_ID,
                &[&user_id, &media_id, &review.rating, &review.text]).await?,
            &MediaSearchKey::TitleAuthor {title, author} =>  self.execute(
                POST_REVIEW_WITH_TITLE_AUTHOR,
                &[&user_id, &title, &author, &review.rating, &review.text]).await?,
        };

        Ok(())
    }

    pub async fn get_author_contacts_named(&self, author: String) -> ResultGetOne<Option<AuthorContacts>> {
        todo!()
    }

    pub async fn get_info(&self, user_id: UserKey) -> ResultGetOne<Option<UserInfo>> {
        self.query_opt::<UserInfo>(GET_INFO, &[&user_id]).await
    }

    pub async fn get_privelegies(&self, user_id: UserKey) -> ResultGetOne<Option<UserPriveleges>> {
        self.query_opt::<UserPriveleges>(GET_PRIVELEGES, &[&user_id]).await
    }
}

mod statements {
    pub const GET_INFO: &str = "get_info";
    pub const GET_PRIVELEGES: &str = "get_privelegies";
    pub const POST_REVIEW_WITH_ID: &str = "post_review_with_id";
    pub const POST_REVIEW_WITH_TITLE_AUTHOR: &str = "post_review_with_title_author";
}



#[async_trait]
impl InitStatements for roles::Registered {
    async fn init_statements(client: &PostgresClient) -> InitStatementsResult {
        use statements::*;
        let mut statements = StatementCollection::new();

        macro_rules! insert_statement {
            ($key:ident, $file_name:literal) => {
                statements.insert(
                    $key, 
                    client.prepare(
                        include_str!(
                            concat!("sql/user/", $file_name)
                        )
                    ).await?
                );
            };
        }

        insert_statement!(GET_INFO, "get_info.sql");
        insert_statement!(GET_PRIVELEGES, "get_privelegies.sql");
        insert_statement!(POST_REVIEW_WITH_ID, "post_review_with_id.sql");
        insert_statement!(POST_REVIEW_WITH_TITLE_AUTHOR, "post_review_with_title_author.sql");

        Ok(statements)
    }
}