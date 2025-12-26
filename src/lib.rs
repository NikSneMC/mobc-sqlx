use std::{marker::PhantomData, str::FromStr as _};

pub use mobc;
use mobc::{Manager, async_trait};
pub use sqlx;
use sqlx::{Connection, Database};

mod migrator;
pub use migrator::SqlxMigrationExt;

pub struct SqlxConnectionManager<DB>
where
    DB: Database + Sync,
{
    connect_options: <DB::Connection as Connection>::Options,
    _phantom: PhantomData<DB>,
}

impl<DB> SqlxConnectionManager<DB>
where
    DB: Database + Sync,
{
    #[must_use]
    pub fn new(
        connect_options: <DB::Connection as Connection>::Options,
    ) -> Self {
        Self {
            connect_options,
            _phantom: PhantomData,
        }
    }

    pub fn from_url(url: &str) -> Result<Self, sqlx::Error> {
        let options = <DB::Connection as Connection>::Options::from_str(url)?;
        Ok(Self::new(options))
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
        Self::Connection::connect_with(&self.connect_options).await
    }

    async fn check(
        &self,
        mut conn: Self::Connection,
    ) -> Result<Self::Connection, Self::Error> {
        conn.ping().await.map(|()| conn)
    }
}
