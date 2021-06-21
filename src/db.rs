use crate::models::OtpToken;
use chrono::{DateTime, Duration, Local};
use deadpool_postgres::Client;
use rand::distributions::Alphanumeric;
use rand::Rng;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::GenericClient;

fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
}

pub async fn login_user(client: &Client, username: &String, password: &String) -> i32 {
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

pub async fn generate_login_session(client: &Client, user: &i32) -> String {
    let token = generate_token();
    let statement = client
        .prepare("insert into login_sessions(session_token, user_id, creation_time, expire_time) values ($1, $2, NOW(), $3)")
        .await
        .unwrap();

    let token_duration = Duration::hours(1);
    let local: DateTime<chrono::Local> = Local::now() + token_duration;

    let _result = client
        .query(&statement, &[&token, &user, &local])
        .await
        .expect("Error executing query on sessions table");

    token
}

pub async fn delete_login_session(client: &Client, token: &OtpToken) {
    let statement = client
        .prepare("delete from login_sessions where session_token = $1")
        .await
        .unwrap();

    let _result = client
        .query(&statement, &[&token.token])
        .await
        .expect("Error executing query on sessions table");
}

pub async fn validate_session_token(client: &Client, token: &OtpToken) -> bool {
    let statement = client
        .prepare("select * from login_sessions where session_token = $1")
        .await
        .unwrap();
    let _result = client
        .query(&statement, &[&token.token])
        .await
        .expect("Error getting token");

    return true;
}

pub async fn get_user_id(client: &Client, username: String) -> i32 {
    let statement = client
        .prepare("select id from users where username = $1")
        .await
        .unwrap();

    let user_db_id = client
        .query(&statement, &[&username])
        .await
        .expect("Error getting user id");

    if user_db_id.len() == 1 {
        return user_db_id.get(0).unwrap().get(0);
    } else {
        return 0;
    }
}

pub async fn get_client_db_id(client: &Client, client_id: String) -> i32 {
    let statement = client
        .prepare("select id from clients where client_id = $1")
        .await
        .unwrap();

    let client_db_id = client
        .query(&statement, &[&client_id])
        .await
        .expect("Error getting client id");

    if client_db_id.len() == 1 {
        return client_db_id.get(0).unwrap().get(0);
    } else {
        return 0;
    }
}

pub async fn generate_authorization_code(
    client: &Client,
    client_db_id: &i32,
    user_id: &i32,
) -> String {
    // Check session token eerst

    // Genereer authorization token
    let token = generate_token();

    let token_duration = Duration::minutes(5);
    let local: DateTime<chrono::Local> = Local::now() + token_duration;

    let statement = client
        .prepare("insert into authorization_codes (client_id, user_id, code, creation_time, expire_time) values ($1, $2, $3, NOW(), $4)")
        .await
        .unwrap();

    let _result = client
        .query(&statement, &[&user_id, &client_db_id, &token, &local])
        .await
        .expect("Error generationg authorization code");

    token
}
