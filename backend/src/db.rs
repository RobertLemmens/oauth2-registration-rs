use crate::models::{OtpToken, Registration};
use chrono::{DateTime, Duration, Local};
use deadpool_postgres::Client;
use rand::distributions::Alphanumeric;
use rand::Rng;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::GenericClient;
use argon2::{self, Config};
use uuid::Uuid;

fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
}

pub async fn login_user(client: &Client, email: &String, password: &String) -> Option<Uuid> {
    let statement = client
        .prepare("select id from users where email = $1 and password = $2")
        .await
        .unwrap();

    let salt = b"yX2SBRQuw%*mpM";
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap();
    let matches = argon2::verify_encoded(&hash, password.as_bytes()).unwrap();
    let user = client
        .query(&statement, &[&email, &hash])
        .await
        .expect("Error executing query on users table");

    if user.len() == 1 {
        return Some(user.get(0).unwrap().get(0));
    } else {
        return None;
    }
}

pub async fn generate_login_session(client: &Client, user: &Uuid) -> String {
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

pub async fn register_user(client: &Client, registration: Registration) -> bool {
    let salt = b"yX2SBRQuw%*mpM";
    let config = Config::default();
    let hash = argon2::hash_encoded(registration.pass.as_bytes(), salt, &config).unwrap();

    let statement = client.prepare("insert into users (username, email, password) values ($1, $2, $3)").await.unwrap();
    let _result = client.query(&statement, &[&registration.user, &registration.email, &hash]).await.expect("Error registering user");

    true
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

pub async fn get_user_id_by_email(client: &Client, email: String) -> Option<Uuid> {
    let statement = client
        .prepare("select id from users where email = $1")
        .await
        .unwrap();

    let user_db_id = client
        .query(&statement, &[&email])
        .await
        .expect("Error getting user id");

    if user_db_id.len() == 1 {
        return Some(user_db_id.get(0).unwrap().get(0));
    } else {
        return None;
    }
}

pub async fn get_user_id_by_username(client: &Client, username: String) -> Option<Uuid> {
    let statement = client
        .prepare("select id from users where username = $1")
        .await
        .unwrap();

    let user_db_id = client
        .query(&statement, &[&username])
        .await
        .expect("Error getting user id");

    if user_db_id.len() == 1 {
        return Some(user_db_id.get(0).unwrap().get(0));
    } else {
        return None;
    }
}

pub async fn get_client_db_id(client: &Client, client_id: String) -> Option<Uuid> {
    let statement = client
        .prepare("select id from clients where client_id = $1")
        .await
        .unwrap();

    let client_db_id = client
        .query(&statement, &[&client_id])
        .await
        .expect("Error getting client id");

    if client_db_id.len() == 1 {
        return Some(client_db_id.get(0).unwrap().get(0));
    } else {
        return None;
    }
}

pub async fn generate_authorization_code(
    client: &Client,
    client_db_id: &Uuid,
    user_id: &Uuid,
    pcke_hash: String,
    device: String
) -> String {
    // Check session token eerst

    // Genereer authorization token
    let token = generate_token();

    let token_duration = Duration::minutes(5);
    let local: DateTime<chrono::Local> = Local::now() + token_duration;

    let statement = client
        .prepare("insert into authorization_codes (client_id, user_id, code, creation_time, expire_time, pcke_hash, device) values ($1, $2, $3, NOW(), $4, $5, $6)")
        .await
        .unwrap();

    let _result = client
        .query(&statement, &[&client_db_id, &user_id, &token, &local, &pcke_hash, &device])
        .await
        .expect("Error generationg authorization code");

    token
}
