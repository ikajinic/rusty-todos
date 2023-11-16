use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub const DB_URL: &str = "sqlite://todos.db";

pub async fn create_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

pub async fn migrate(pool: &SqlitePool) {
    let migrations_location = std::path::Path::new("./migrations");
    match sqlx::migrate::Migrator::new(migrations_location)
        .await
        .unwrap()
        .run(pool)
        .await {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
}
