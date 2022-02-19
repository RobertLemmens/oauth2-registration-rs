use yew::{Component, Context, Html, html};
use yew_router::prelude::*;
use crate::login::Authorization;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;
use serde_json::json;
use serde::{Deserialize, Serialize};

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
                <h1 class="title is-2 note-fg-dark">{"Authorize Leafpad"}</h1>
                <div class="content">
                    <p class="subtitle is-4">{"Read notes"}</p>
                    <p class="subtitle is-4">{"Write notes"}</p>
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
        let history = ctx.link().history().unwrap();
        match msg {
            Msg::AuthorizeEvent => {
                authorize_action(self.authorization.clone(),history);
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

fn authorize_action(authorization: Authorization, history: AnyHistory) {
    spawn_local(async move {
        authorize(authorization, history).await;
    })
}

#[derive(Serialize, Deserialize)]
struct AuthorizeRequest {
    device: String,
    username: String,
    client_id: String,
    pcke: String
}

async fn authorize(authorization: Authorization, history: AnyHistory) {
    let auth_request = AuthorizeRequest {
        device: authorization.device,
        username: authorization.username,
        client_id: authorization.client_id,
        pcke: authorization.pcke
    };
    let resp = Request::post("/server/api/v1/login")
        .header("Content-Type", "application/json")
        .body(json!(auth_request).to_string())
        .send()
        .await;

    match resp {
        Ok(response) => {
            match response.status() {
                200 => {
                    log::info!("Got OK from authorize service")
                }
                _ => {log::info!("Non OK returned from authorize service")}
            }
        },
        Err(e) => {log::info!("Error authorizing request {:?}", e)}
    }
    


}
