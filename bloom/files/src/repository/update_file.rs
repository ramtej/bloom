use super::Repository;
use crate::{entities::File, Error};
use kernel::db::Queryer;

impl Repository {
    pub async fn update_file<'c, C: Queryer<'c>>(&self, db: C, file: &File) -> Result<(), Error> {
        todo!();
    }
}