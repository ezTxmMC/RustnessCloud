use sqlx::mysql::MySqlPoolOptions;

async fn connect_to_mysql(host: &str, database: &str, username: &str, password: &str) -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&format!("mysql://{}:{}@{}/{}", username, password, host, database))
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT 1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}