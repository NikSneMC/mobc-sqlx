use mobc::{Pool, async_trait};
use sqlx::{
    Database,
    migrate::{Migrate, Migrator},
};

use crate::SqlxConnectionManager;

#[async_trait]
pub trait SqlxMigrationExt<DB>
where
    DB: Database + Sync,
    <DB as Database>::Connection: Migrate,
{
    async fn migrate(
        &self,
        migrator: Migrator,
    ) -> Result<(), mobc::Error<sqlx::Error>>;
}

#[async_trait]
impl<DB> SqlxMigrationExt<DB> for Pool<SqlxConnectionManager<DB>>
where
    DB: Database + Sync,
    <DB as Database>::Connection: Migrate,
{
    async fn migrate(
        &self,
        migrator: Migrator,
    ) -> Result<(), mobc::Error<sqlx::Error>> {
        let mut connection = self.get().await?;

        migrator
            .run_direct(&mut *connection)
            .await
            .map_err(sqlx::Error::from)?;

        Ok(())
    }
}
