use super::{
    Priveleges,
    roles::Author
};

use crate::db::{
    result::*,
    model::{
        ModerationSearchOptions
    }
};

use keter_media_model::{media::*, usage::LicenseSearchKey, userinfo::*};

impl Priveleges<Author> {
    pub async fn create_media(&self, reg_media: &RegisterMedia) -> ResultSelectOne<MediaKey> {
        self.client.create_media(self.user_key, reg_media).await
    }

    pub async fn insert_material(
        &self,
        media_id: MediaKey,
        license: LicenseSearchKey,
        format: &str,
        quality: Quality,
    ) -> ResultSelectOne<MaterialKey> {
        self.client.insert_material(self.user_key, media_id, license, &format, quality).await
    }

    pub async fn delete_material(&self, material_id: MaterialKey) -> ResultDeleteOne {
        self.client.delete_material(self.user_key, material_id).await
    }
}