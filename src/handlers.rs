use crate::models::{AuthorizationCode, AuthorizationParams, OtpToken};
use crate::{db, response::Response, Credentials};
use deadpool_postgres::Client;
use warp::{Rejection, Reply};

pub async fn get_health() -> Response {
    Ok(warp::reply::json(&"UP"))
}

pub async fn get_authorization_code(
    token: OtpToken,
    params: AuthorizationParams,
    db_pool: deadpool_postgres::Pool,
) -> Response {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    // TODO check tijd op session token
    if db::validate_session_token(&client, &token).await {
        let client_db_id = db::get_client_db_id(&client, params.client_id).await;
        let user_db_id = db::get_user_id(&client, params.username).await; //TODO is dit nodig? Retrieve uit token beter? exposen de username zonder reden?
        // TODO check if client id of user id == 0, dan kunnen we meteen stoppen
        let result = db::generate_authorization_code(&client, &client_db_id, &user_db_id, params.pcke, params.device).await;
        let code = AuthorizationCode { code: result };
        let _del_result = db::delete_login_session(&client, &token).await;
        return Ok(warp::reply::json(&code));
    }

    let code = AuthorizationCode {
        code: "".to_owned(),
    };
    Ok(warp::reply::json(&code))
}

pub async fn login(
    credentials: Credentials,
    db_pool: deadpool_postgres::Pool,
) -> std::result::Result<impl Reply, Rejection> {
    println!("{}", credentials.user);
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let user_id = db::login_user(&client, &credentials.user, &credentials.pass).await;

    if user_id > 0 {
        let token = db::generate_login_session(&client, &user_id).await; // one time use token, should be deleted after an authorization token is created
        let token_response = OtpToken {
            token: token.to_owned(),
        };
        let token_string = serde_json::to_string(&token_response);
        return match token_string {
            Ok(s) => Ok(warp::http::Response::builder().status(200).body(s)),
            Err(_) => Ok(warp::http::Response::builder()
                .status(500)
                .body("Something went wrong".to_owned())),
        };
    }

    // because i dont get error handling in warp yet, we use response builder here.
    Ok(warp::http::Response::builder()
        .status(401)
        .body("Incorrect username or password".to_owned()))
}
