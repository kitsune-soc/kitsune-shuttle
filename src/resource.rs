use async_trait::async_trait;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use serde::Serialize;
use shuttle_runtime::ResourceBuilder;
use shuttle_service::{
    database::{SharedEngine, Type},
    DatabaseReadyInfo, Factory,
};

#[derive(Serialize)]
pub struct PostgresResource;

#[async_trait]
impl ResourceBuilder<DatabaseConnection> for PostgresResource {
    const TYPE: shuttle_service::Type =
        shuttle_service::Type::Database(Type::Shared(SharedEngine::Postgres));

    type Output = DatabaseReadyInfo;

    fn new() -> Self {
        Self
    }

    async fn build(conn_data: &Self::Output) -> Result<DatabaseConnection, shuttle_service::Error> {
        let db_conn = Database::connect(conn_data.connection_string_public())
            .await
            .map_err(|err| shuttle_service::Error::Custom(err.into()))?;

        Migrator::up(&db_conn, None)
            .await
            .map_err(|err| shuttle_service::Error::Custom(err.into()))?;

        Ok(db_conn)
    }

    async fn output(
        self,
        factory: &mut dyn Factory,
    ) -> Result<Self::Output, shuttle_service::Error> {
        let conn_data = factory
            .get_db_connection(Type::Shared(SharedEngine::Postgres))
            .await?;

        Ok(conn_data)
    }
}
