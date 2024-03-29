use std::fmt::{Display, Formatter};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

use reqwasm::http::*;

use wasm_bindgen_futures::spawn_local;
use wasm_logger::*;
use yew::html::Scope;
use yew::prelude::*;
use yew::{events, events::Event, Component, Context, Html};

use serde::{Deserialize, Serialize};
use serde_json::json;
use yew_router::hooks::use_history;
use yew_router::prelude::*;

use crate::login::InputField::{Password, Username};
use crate::{Authorize, Route};

pub struct Login {
    username: String,
    password: String,
    login_err: bool
}

pub enum InputField {
    Username,
    Password,
}

pub enum Msg {
    LoginEvent,
    InputEvent(String, InputField),
}

impl Login {
    fn login_card(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        // Login triggers
        let onclick = link.callback(|_| Msg::LoginEvent);
        let onkeypress = ctx.link().batch_callback(|event: KeyboardEvent| {
            if event.key() == "Enter" {
                Some(Msg::LoginEvent)
            } else {
                None
            }
        });

        // Input events
        let oninput = ctx.link().batch_callback(|event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| Msg::InputEvent(input.value(), Username))
        });

        let oninput_pass = ctx.link().batch_callback(|event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| Msg::InputEvent(input.value(), Password))
        });

        html! {
                   <div class="box">
                    <h1 class="title is-2 note-fg-dark">{"Login"}</h1>
                     <div class="field">
                         <div class="control has-icons-left">
                             <input onchange={oninput} class="input is-medium" placeholder="Email address" type="text" />
                             <span class="icon is-small is-left">
                               <i class="fas fa-user"></i>
                             </span>
                         </div>
                     </div>
                     <div class="field">
                         <div class="control has-icons-left">
                             <input onchange={oninput_pass} class="input is-medium" placeholder="Password" type="password" />
                             <span class="icon is-small is-left">
                               <i class="fas fa-lock"></i>
                             </span>
                         </div>
                     </div>
                     <button {onclick} class="button notes-bg-dark is-rounded is-medium"><span>{"Login"}</span>
                        <span class="icon is-small">
                           <i class="fas fa-arrow-right"></i>
                        </span>
                     </button>
                   </div>
        }
    }
}

fn login_action(username: String, password: String, history: AnyHistory, location: AnyLocation) {
    let user = username.clone();
    let pass = password.clone();
    spawn_local(async move {
        login(user, pass, history, location).await;
    })
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
            login_err: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoginEvent => {
                let history = ctx.link().history().unwrap();
                let location = ctx.link().location().unwrap();
                login_action(self.username.clone(), self.password.clone(), history, location);
                true
            }
            Msg::InputEvent(val, typ) => {
                match typ {
                    Username => self.username = val,
                    Password => self.password = val,
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

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
                                          { self.login_card(ctx)}
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
                              <h1 class="title is-2 has-text-white-ter">{"Dont have an account yet?"}</h1>
                              <Link<Route> to={Route::Register} classes="button is-white is-rounded is-medium">{"Create one here"}</Link<Route>>
                            </div>
                          </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    user: String,
    pass: String,
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Authorization {
    pub device: String,
    pub pcke: String,
    pub otp: String,
    pub client_id: String,
    pub username: String,
    pub redirect_uri: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequestParams {
    pub device: String,
    pub client_id: String,
    pub pcke: String,
    pub redirect_uri: String
}

impl Display for TokenResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(token:{})", self.token)
    }
}

async fn login(username: String, password: String, history: AnyHistory, location: AnyLocation) {
    let body = format!("user: {}, pass: {}", username, password);
    let credentials = Credentials {
        user: username,
        pass: password,
    };
    
    let query_params = match location.query::<LoginRequestParams>() {
        Ok(params) => params,
        Err(e) => {
            log::info!("Error, no query params, cant continue");
            LoginRequestParams {
                device: "".to_string(),
                client_id: "".to_string(),
                pcke: "".to_string(),
                redirect_uri: "".to_string()
            }
        }
    };
    let resp = Request::post("/server/api/v1/login")
        .header("Content-Type", "application/json")
        .body(json!(credentials).to_string())
        .send()
        .await;
    match resp {
        Ok(resp) => match resp.status() {
            200 => {
                log::info!("Logged in! ");
                match resp.json::<TokenResponse>().await {
                    Ok(js) => {
                        log::info!("{}", js);
                        let auth_params = Authorization {
                            device: query_params.device,
                            pcke: query_params.pcke,
                            otp: js.token,
                            client_id: query_params.client_id,
                            username: credentials.user,
                            redirect_uri: query_params.redirect_uri
                        };
                        history.push_with_query(Route::Authorize, auth_params);
                    }
                    Err(e) => log::info!("Error parsing json response {:?}", e),
                }
            }
            _ => {
                // let login_error = link.callback(|_| Msg::LoginEvent);
                log::info!("Error...")
            }
        },
        Err(e) => {
            log::info!("Error {:?}", e)
        }
    }
}
