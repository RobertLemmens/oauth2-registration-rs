use std::convert::Infallible;
use crate::models::{AuthorizationCode, AuthorizationParams, OtpToken, Registration, Credentials};
use crate::{db, response::Response};
use deadpool_postgres::Client;
use warp::{reject, Rejection, Reply};
use warp::http::StatusCode;
use serde::Serialize;

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
        println!("Getting id for {}", params.username);
        let user_db_id = db::get_user_id_by_email(&client, params.username).await; //TODO is dit nodig? Retrieve uit token beter? exposen de username zonder reden?
        // TODO check if client id of user id == 0, dan kunnen we meteen stoppen
        if user_db_id == 0 {
            println!("Returned ID is 0. Could not find user email");
        } else {
            println!("Retuend ID is {}", user_db_id);
        }
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

    Err(reject::custom(Unauthorized))
    // because i dont get error handling in warp yet, we use response builder here.
    // Ok(warp::http::Response::builder()
    //     .status(401)
    //     .body("Incorrect username or password".to_owned()))
}

pub async fn register(registration: Registration, db_pool: deadpool_postgres::Pool) -> std::result::Result<impl Reply, Rejection> {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let success = db::register_user(&client, registration).await;

    Ok(warp::http::Response::builder()
        .status(200)
        .body("This email adress is already taken".to_owned()))
}

#[derive(Debug)]
struct Unauthorized;
impl reject::Reject for Unauthorized {}
/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(Unauthorized) = err.find() {
        code = StatusCode::UNAUTHORIZED;
        message = "Incorrect credentials"
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
