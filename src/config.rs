use std::env;

pub fn get_connection_string() -> String {
    let user = env::var("POSTGRES_USER").expect("Environment missing POSTGRES_USER variable.");
    let pw = env::var("POSTGRES_PASSWORD").expect("Environment missing POSTGRES_PASSWORD variable.");
    let conn_str = format!("postgres://{}:{}@localhost/postgres", user, pw);
    conn_str
}
