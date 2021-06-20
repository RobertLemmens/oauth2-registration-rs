use chrono::{DateTime, Duration, Local};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn login_user(
    client: &Client,
    username: String,
    password: String,
) -> i32 {
    let statement = client
        .prepare("select id from users where username = $1 and password = $2")
        .await
        .unwrap();

    let user = client
        .query(&statement, &[&username, &password])
        .await
        .expect("Error executing query on users table");

    if user.len() == 1 {
        return user.get(0).unwrap().get(0);
    } else {
        return 0;
    }
}
