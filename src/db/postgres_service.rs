use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::info;

#[derive(Clone)]
pub struct PostgresService {
    pub db: DatabaseConnection,
}

impl PostgresService {
    pub async fn new(uri: &str) -> Result<Self, DbErr> {
        info!("Connecting to PostgreSQL...");
        println!("Connecting to PostgreSQL...");
        let db = Database::connect(uri).await?;
        println!("Connected to PostgreSQL.");
        info!("Running migrations...");
        println!("Running migrations...");
        Migrator::up(&db, None).await?;
        println!("Migrations finished.");
        info!("Connected to PostgreSQL.");
        println!("Connected to PostgreSQL.");
        Ok(Self { db })
    }
}
