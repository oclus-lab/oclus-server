use actix_web::{App, HttpServer};
use oclus_server::config::OclusConfig;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = OclusConfig::from_file("config.yaml")?;

    HttpServer::new(|| App::new())
        .bind(config.bind_addr)?
        .run()
        .await?;

    Ok(())
}
