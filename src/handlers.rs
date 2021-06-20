use crate::{Credentials, db, response::Response};
use deadpool_postgres::Client;
use warp::reply::json;

pub async fn get_health() -> Response {
    Ok(warp::reply::json(&"UP"))
}

pub async fn login(credentials: Credentials, db_pool: deadpool_postgres::Pool) -> Response {
    println!("{}", credentials.user);
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let user_id = db::login_user(
        &client,
        credentials.user,
        credentials.pass,
    )
    .await;

    if user_id > 0 {
        return Ok(json(&"Done"));
    }

    Ok(json(&"Error"))
}
