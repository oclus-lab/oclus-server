use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use config::Config;
use log::SetLoggerError;
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::io;

pub mod config;
mod domain;

#[derive(Clone, Debug)]
pub struct AppState {
    config: Config,
    db: PgPool,
}

impl AppState {
    pub fn new(config: Config, db: PgPool) -> Self {
        Self { config, db }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn db(&self) -> &PgPool {
        &self.db
    }
}

pub async fn run(config: Config, db: PgPool) -> Result<(), io::Error> {
    let bind_addr = config.bind_addr.clone();

    let state = AppState::new(config, db);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(middleware::Logger::default())
    })
    .bind(bind_addr)?
    .run()
    .await
}

pub async fn init_db(config: &Config) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new().connect(&config.db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

pub fn init_log(config: &Config) -> Result<(), SetLoggerError> {
    CombinedLogger::init(vec![TermLogger::new(
        config.log_level,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
}
