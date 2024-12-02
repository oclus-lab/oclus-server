use oclus_server::config::Config;
use oclus_server::{init_db, init_log, run, AppState};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_file("./config.yaml")?;
    let db = init_db(&config).await?;

    init_log(&config)?;

    run(config, db).await?;

    Ok(())
}
