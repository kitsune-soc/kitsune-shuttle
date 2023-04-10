use self::resource::PostgresResource;
use kitsune::config::Configuration;
use sea_orm::DatabaseConnection;
use shuttle_secrets::{SecretStore, Secrets};
use shuttle_static_folder::StaticFolder;
use std::path::PathBuf;

mod resource;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[shuttle_runtime::main]
async fn axum(
    #[PostgresResource] db_conn: DatabaseConnection,
    #[Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let config: Configuration = serde_dhall::from_str(&secrets.get("config").unwrap())
        .static_type_annotation()
        .parse()
        .map_err(anyhow::Error::from)?;
    let state = kitsune::initialise_state(&config, db_conn).await;
    let router = kitsune::http::create_router(state.clone(), &config.server);

    tokio::spawn(kitsune::job::run_dispatcher(
        state,
        config.server.job_workers,
    ));

    Ok(router.into())
}
