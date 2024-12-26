use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use oclus_server::api::route;
use oclus_server::config::Config;
use oclus_server::infra::db::init_db;
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode};
use oclus_server::service::ServiceCollection;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = Config::from_yaml("./config.yaml")?;

    CombinedLogger::init(vec![TermLogger::new(
        config.log_level,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])?;

    let db = init_db(&config.db_url).await?;
    let services = ServiceCollection::new(&config, db)?;

    HttpServer::new(move || {
        App::new()
            .configure(route::define_routes)
            .app_data(Data::new(services.clone()))
            .app_data(Data::new(config.clone()))
            .wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
