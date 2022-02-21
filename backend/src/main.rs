mod handlers;
mod models;
mod response;
mod db;

extern crate argon2;

use tokio_postgres::NoTls;
use dotenv::dotenv;
use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddrV4};
use warp::Filter;
use crate::models::{AuthorizationParams, Config, Credentials, Registration, OtpToken, ServerConfig};

fn with_db(
    db_pool: deadpool_postgres::Pool,
) -> impl Filter<Extract = (deadpool_postgres::Pool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

fn with_config(
    config: Config,
) -> impl Filter<Extract = (ServerConfig,), Error = Infallible> + Clone {
    warp::any().map(move || config.server.clone())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config: Config = crate::models::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    let client_user_params = warp::query().map(|params: AuthorizationParams| {
        params
    });

    println!(
        "Starting registration server on http://{}:{}/",
        "0.0.0.0", config.server.port
    );

    let server_base = warp::path("server")
        .and(warp::path("api"))
        .and(warp::path("v1"));

    let static_route = warp::get()
        .and(warp::fs::dir("static"));

    let static_route_2 = warp::get()
        .and(warp::path("login"))
        .and(warp::fs::dir("static"));

    let static_route_3 = warp::get()
        .and(warp::path("register"))
        .and(warp::fs::dir("static"));

    let static_route_4 = warp::get()
        .and(warp::path("authorize"))
        .and(warp::fs::dir("static"));

    let static_route_5 = warp::get()
        .and(warp::path("success"))
        .and(warp::fs::dir("static"));

    let login_route = warp::post()
        .and(server_base)
        .and(warp::path("login"))
        .and(warp::body::json())
        .map(|credentials: Credentials| credentials)
        .and(with_db(pool.clone()))
        .and_then(handlers::login);

    let register_route = warp::post()
        .and(server_base)
        .and(warp::path("register"))
        .and(warp::body::json())
        .map(|registration: Registration| registration)
        .and(with_db(pool.clone()))
        .and_then(handlers::register);

    let authorize_route = warp::post()
        .and(server_base)
        .and(warp::path("authorize"))
        .and(warp::body::json())
        .map(|token: OtpToken| token)
        .and(client_user_params)
        .and(with_db(pool.clone()))
        .and_then(handlers::get_authorization_code);

    let health_route = warp::get()
        .and(warp::path("q"))
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(handlers::get_health);

    let routes =
        static_route
            .or(static_route_2)
            .or(static_route_3)
            .or(static_route_4)
            .or(static_route_5)
            .or(health_route)
            .or(login_route).or(register_route).or(authorize_route).recover(handlers::handle_rejection);

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), config.server.port);
    warp::serve(routes).run(addr).await;
}