use log::LevelFilter;
use oclus_server::config::Config;
use oclus_server::infra::db::init_db;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use url::Url;
use uuid::Uuid;
use oclus_server::service::ServiceCollection;

#[derive(Clone)]
pub struct TestEnv {
    pub config: Config,
    pub services: ServiceCollection,
    pub db: Pool<Postgres>,
}

impl TestEnv {
    pub async fn new() -> Self {
        let admin_db_url = env::var("ADMIN_DB_URL").expect("ADMIN_DB_URL");
        let admin_db = PgPoolOptions::new().connect(&admin_db_url).await.unwrap();

        let db_name = format!("test_{}", Uuid::new_v4());
        sqlx::query(&format!("CREATE DATABASE \"{}\"", db_name))
            .execute(&admin_db)
            .await
            .unwrap();

        let mut db_url = Url::parse(&admin_db_url).unwrap();
        db_url.set_path(&db_name);
        let db = init_db(&db_url).await.unwrap();
        
        sqlx::migrate!("./migrations").run(&db).await.unwrap();

        let config = Config {
            log_level: LevelFilter::Off,
            otp_secret: "very_strong_secret".to_string(),
            token_secret: "very_strong_secret".to_string(),
            db_url,
        };

        let services = ServiceCollection::new(&config, db.clone()).unwrap();

        Self {
            config,
            db,
            services,
        }
    }
}
