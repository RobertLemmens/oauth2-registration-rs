use yew::{Component, Context, Html, html};
use yew_router::prelude::*;
use crate::login::Authorization;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;
use serde_json::json;
use serde::{Deserialize, Serialize};
use web_sys::window;

pub struct Authorize {
    authorization: Authorization
}

pub enum Msg {
    AuthorizeEvent,
}

impl Authorize {
    fn authorize_card(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link(); 
        let onclick = link.callback(|_| Msg::AuthorizeEvent);

        html! {
               <div class="box">
                <h1 class="title is-2 note-fg-dark">{"Authorize"}</h1>
                <div class="content">
                    <p class="subtitle is-4">{"Read"}</p>
                    <p class="subtitle is-4">{"Write"}</p>
                    <p class="subtitle is-4">{"Profile"}</p>
                </div>
                 <button {onclick} class="button notes-bg-dark is-rounded is-medium"><span>{"Authorize"}</span>
                    <span class="icon is-small">
                       <i class="fas fa-check"></i>
                    </span>
                 </button>
               </div>
        }
    }
}

impl Component for Authorize {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let location = ctx.link().location().unwrap().query::<Authorization>();
        let auth = match location {
            Ok(authorization) => {
                log::info!("Authorization {:?}", authorization);
                authorization
            },
            Err(e) => {
                log::info!("Error, missing parameters {:?}", e);
                Authorization {
                    username: "".to_string(),
                    device: "".to_string(),
                    pcke: "".to_string(),
                    otp: "".to_string(),
                    client_id: "".to_string(),
                    redirect_uri: "".to_string()
                }
            }
        };

        Self {
            authorization: auth
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let location = ctx.link().location().unwrap();
        match msg {
            Msg::AuthorizeEvent => {
                authorize_action(self.authorization.clone(),location);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="login-wrapper is-vertical-center">
                <div class="columns">
                    <div class="column is-two-thirds">

                        <div class="columns is-centered">
                            <div class="column is-half">
                                <div class="login-form">
                                    <section class="hero is-fullheight">
                                      <div class="hero-body has-text-centered">
                                        <div class="container">
                                          { self.authorize_card(ctx)}
                                        </div>
                                      </div>
                                    </section>
                                </div>
                            </div>
                       </div>
                    </div>
                    <div class="column note-bg-dark">
                        <div class="hero is-fullheight">
                          <div class="hero-body has-text-centered">
                            <div class="container">
                              <figure class="image has-image-centered">
                                <img style="width:300px; margin-left:auto; margin-right:auto;" src="assets/notes_logo.png"/>
                              </figure>
                            </div>
                          </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn authorize_action(authorization: Authorization, location: AnyLocation) {
    spawn_local(async move {
        authorize(authorization, location).await;
    })
}

#[derive(Serialize, Deserialize)]
struct AuthorizeRequest {
    device: String,
    username: String,
    client_id: String,
    pcke: String
}

#[derive(Serialize, Deserialize)]
struct AuthorizeResponse {
    code: String
}

#[derive(Serialize, Deserialize)]
struct AuthorizePayload {
    token: String
}

async fn authorize(authorization: Authorization, location: AnyLocation) {
    let auth_payload = AuthorizePayload {
        token: authorization.otp
    };
    let uri = format!("/server/api/v1/authorize?device={}&username={}&client_id={}&pcke={}", authorization.redirect_uri.to_owned(), authorization.username.to_owned(), authorization.client_id.to_owned(), authorization.pcke.to_owned());
    let resp = Request::post(&uri)
        .header("Content-Type", "application/json")
        .body(json!(auth_payload).to_string())
        .send()
        .await;

    match resp {
        Ok(response) => {
            match response.status() {
                200 => {
                    // We krijgen [code] terug, en moeten dan [token] opsturen. TODO fix dat..
                    log::info!("Got OK from authorize service");
                    match response.json::<AuthorizeResponse>().await {
                        Ok(code) => {
                            let uri = format!("{}?token={}", authorization.redirect_uri.to_owned(), code.code.to_owned());
                            let win = window().unwrap();
                            win.location().set_href(&uri);
                            // got code back, redirect back to redirect uri with this code
                        }
                        Err(e) => {log::error!("Error deserializing response body {}", e)}
                    }
                }
                _ => {log::info!("Non OK returned from authorize service")}
            }
        },
        Err(e) => {log::info!("Error authorizing request {:?}", e)}
    }
    


}
