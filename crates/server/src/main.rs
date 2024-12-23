use actix_web::web::Data;
use actix_web::{App, HttpServer};
use oclus_server::api::route;
use oclus_server::config::Config;
use oclus_server::db::init_database;
use oclus_server::db::repo::RepoCollection;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = Config::from_yaml("./config.yaml")?;
    let database = init_database(&config.database_url).await?;

    let repo_collection = RepoCollection::new(database);

    HttpServer::new(move || {
        App::new()
            .configure(route::define_routes)
            .app_data(Data::new(repo_collection.clone()))
            .app_data(Data::new(config.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
