use actix_web::web::Data;
use actix_web::{App, HttpServer};
use oclus_server::config::OclusConfig;
use oclus_server::infra::repo::OclusDatabase;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = OclusConfig::from_file("config.yaml")?;
    let db = OclusDatabase::new(&config.db_url).await?;

    HttpServer::new(move || App::new().app_data(Data::new(db.clone())))
        .bind(config.bind_addr)?
        .run()
        .await?;

    Ok(())
}
