use std::marker::PhantomData;

pub use mobc;
pub use sqlx;

use mobc::{Manager, async_trait};
use sqlx::{Connection as _, Database};

mod migrator;
pub use migrator::SqlxMigrationExt;

pub struct SqlxConnectionManager<DB>
where
    DB: Database + Sync,
{
    url: String,
    _phantom: PhantomData<DB>,
}

impl<DB> SqlxConnectionManager<DB>
where
    DB: Database + Sync,
{
    #[must_use]
    pub fn new<S>(url: S) -> Self
    where
        S: ToString,
    {
        Self {
            url: url.to_string(),
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<DB> Manager for SqlxConnectionManager<DB>
where
    DB: Database + Sync,
{
    type Connection = DB::Connection;
    type Error = sqlx::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Self::Connection::connect(&self.url).await
    }

    async fn check(
        &self,
        mut conn: Self::Connection,
    ) -> Result<Self::Connection, Self::Error> {
        conn.ping().await.map(|()| conn)
    }
}
