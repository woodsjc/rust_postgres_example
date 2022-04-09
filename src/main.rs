mod sqlx_db;
mod tokio_db;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //tokio version
    tokio_db::tokio_db().await?;
    sqlx_db::sqlx_db().await?;

    Ok(())
}
