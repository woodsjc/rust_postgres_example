use chrono::prelude::{DateTime, Utc};
use std::{fmt, time::SystemTime};
use tokio_postgres::{NoTls, Error, Client, Row};


struct DB {
    client: Client,
}

impl DB {
    pub async fn new(conn_str: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(Self{client})
    }

    pub async fn create_table(&self, table: &str, columns: &str) -> Result<(), Error> {
        let create_table_query = format!("CREATE TABLE IF NOT EXISTS {} ({});", table, columns);
        println!("Running create table: \n{}", create_table_query);
        self.client.execute(&create_table_query, &[]).await?;
        Ok(())
    }

    pub async fn simple_query(&self, query: &str) -> Result<Vec<tokio_postgres::Row>, Error> {
        Ok(self.client.query(query, &[]).await?)
    }
}

struct Clients {
    id: i32,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl fmt::Display for Clients {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id:{}, name:{}, created_at:{}, updated_at:{}", self.id, self.name, self.created_at, self.updated_at)
    }
}

impl From<&Row> for Clients {
    fn from(row: &Row) -> Self {
        let created_at: SystemTime = row.get("created_at");
        let updated_at: SystemTime = row.get("updated_at");
        Self {
            id: row.get("id"),
            name: row.get("name"),
            created_at: DateTime::<Utc>::from(created_at),
            updated_at: DateTime::<Utc>::from(updated_at),
        }
    }
}

pub async fn tokio_db(conn_str: &str) -> Result<(), Error> {
    let db = DB::new(conn_str).await?;

    db.create_table(
        "clients",
        "
id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY
, name varchar(255) NOT NULL
, created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
, updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
"
    ).await?;


    let rows = db.simple_query("SELECT 1;").await?;
    let result: i32 = rows[0].get(0);
    println!("{}", result);

    db.client.execute("
INSERT INTO clients (name) VAlUES
($1)
;",
        &[&"first_client"]).await?;

    let rows = db.simple_query("SELECT * FROM clients;").await?;
    for r in rows.iter() {
        println!("{}", Clients::from(r));
    }

    Ok(())
}
