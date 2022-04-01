mod tokio_db;

use tokio_postgres::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //tokio version
    tokio_db::tokio_db().await?;

    Ok(())
}
