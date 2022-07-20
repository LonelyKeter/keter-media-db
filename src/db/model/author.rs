use super::*;

use crate::auth::roles;

use keter_media_model::{media::*, usage::*, userinfo::*};
use postgres_types::Type;
use tokio_postgres::{Row, Statement};

impl Client<roles::Author> {
    pub async fn create_media(
        &self,
        user_id: UserKey,
        reg_media: &RegisterMedia,
    ) -> ResultSelectOne<MediaKey> {
        self.query_val(
            Statements::CreateMedia,
            &[&user_id, &reg_media.title, &reg_media.kind],
        )
        .await?
        .extract()
    }

    pub async fn insert_material(
        &self,
        user_id: UserKey,
        media_id: MediaKey,
        license: LicenseSearchKey,
        format: &str,
        quality: Quality,
    ) -> ResultSelectOne<MaterialKey> {
        match license {
            LicenseSearchKey::Id(license_id) => self
                .query_val(
                    Statements::InsertMaterialLicenseId,
                    &[&user_id, &media_id, &license_id, &format, &quality],
                )
                .await?
                .extract(),
            _ => unimplemented!("Material post with license name"),
        }
    }

    pub async fn delete_material(
        &self,
        user_id: UserKey,
        material_id: MaterialKey,
    ) -> ResultDeleteOne {
        self.execute(Statements::DeleteMaterial, &[&user_id, &material_id])
            .await?;
        Ok(())
    }

    //TODO: Abstract license key
    //TODO: Should update/insert operations require id for stability
    pub async fn update_license_media(
        &self,
        media: MediaSearchKey,
        license: String,
    ) -> ResultUpdateOne<()> {
        todo!()
    }

    pub async fn update_license_material(
        &self,
        material: MaterialKey,
        license: String,
    ) -> ResultUpdateOne<()> {
        todo!()
    }
}

use enum_map::{enum_map, Enum};
#[derive(Enum, Clone, Copy, Debug)]
pub enum Statements {
    InsertMaterialLicenseId,
    DeleteMaterial,
    CreateMedia,
}

#[async_trait]
impl InitStatements for roles::Author {
    type StatementKey = Statements;

    async fn init_statements(client: &PostgresClient) -> InitStatementsResult<Statements> {
        let statements = enum_map! {
            Statements::InsertMaterialLicenseId => client.prepare_typed(
                include_str!("sql/author/insert_material_license_id.sql"),
                &[UserKey::SQL_TYPE, MediaKey::SQL_TYPE, LicenseKey::SQL_TYPE, String::SQL_TYPE]
            ).await
            .map_err(|error| InitStatementsError {statement_key: Statements::InsertMaterialLicenseId, error})?,
            Statements::DeleteMaterial => client.prepare_typed(
                include_str!("sql/author/delete_material.sql"),
                &[UserKey::SQL_TYPE, MaterialKey::SQL_TYPE]
            ).await
            .map_err(|error| InitStatementsError {statement_key: Statements::DeleteMaterial, error})?,
            Statements::CreateMedia => client.prepare_typed(
                include_str!("sql/author/create_media.sql"),
                &[UserKey::SQL_TYPE, String::SQL_TYPE]
            ).await
            .map_err(|error| InitStatementsError {statement_key: Statements::CreateMedia, error})?,
        };

        Ok(statements)
    }
}
