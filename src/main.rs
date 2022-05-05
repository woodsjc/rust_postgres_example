mod config;
mod sqlx_db;
mod tokio_db;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str: String = config::get_connection_string();
    tokio_db::tokio_db(&conn_str).await?;
    sqlx_db::sqlx_db(&conn_str).await?;

    Ok(())
}
