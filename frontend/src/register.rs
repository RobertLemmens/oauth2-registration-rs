use yew::{events, events::Event, Component, Context, Html};
use yew::prelude::*;
use yew::html::Scope;
use yew_router::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use crate::register::InputField::{Name, Email, Password, RepeatPassword};
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;
use serde_json::json;
use serde::{Deserialize, Serialize};

pub struct Register {
    name: String,
    email: String,
    password: String,
    repeat_password: String,
    registration_success: bool
}

pub enum InputField {
    Name,
    Email,
    Password,
    RepeatPassword,
}

pub enum Msg {
    RegisterEvent,
    InputEvent(String, InputField),
    RegisteredEvent(bool)
}

impl Register {

    fn register_card(&self, ctx: &Context<Self>) -> Html{
        let link = ctx.link();
        // Input events
        let onclick = link.callback(|_| Msg::RegisterEvent);

        let email_input = ctx.link().batch_callback(|event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| Msg::InputEvent(input.value(), Email))
        });
        
        let username_input = ctx.link().batch_callback(|event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| Msg::InputEvent(input.value(), Name))
        });

        let password_input = ctx.link().batch_callback(|event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| Msg::InputEvent(input.value(), Password))
        });

        let repeat_password_input = ctx.link().batch_callback(|event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| Msg::InputEvent(input.value(), RepeatPassword))
        });

        if self.registration_success {
            html! {
                <div class="box">
                    <h1 class="title is-2 note-fg-dark">{"Successfully registered!"}</h1>
                </div>
            }
        } else {
            html! {
                   <div class="box">
                    <h1 class="title is-2 note-fg-dark">{"Register"}</h1>
                     <div class="field">
                         <div class="control has-icons-left">
                             <input onchange={email_input} class="input is-medium" placeholder="Email address" type="email" />
                             <span class="icon is-small is-left">
                               <i class="fas fa-envelope"></i>
                             </span>
                         </div>
                     </div>
                     <div class="field">
                         <div class="control has-icons-left">
                             <input onchange={username_input} class="input is-medium" placeholder="Full Name" type="text" />
                             <span class="icon is-small is-left">
                               <i class="fas fa-user"></i>
                             </span>
                         </div>
                     </div>
                     <div class="field">
                         <div class="control has-icons-left">
                             <input onchange={password_input} class="input is-medium" placeholder="Password" type="password" />
                             <span class="icon is-small is-left">
                               <i class="fas fa-lock"></i>
                             </span>
                         </div>
                     </div>
                     <div class="field">
                         <div class="control has-icons-left">
                             <input onchange={repeat_password_input} class="input is-medium" placeholder="Repeat password" type="password" />
                             <span class="icon is-small is-left">
                               <i class="fas fa-lock"></i>
                             </span>
                         </div>
                     </div>
                     <button {onclick} class="button notes-bg-dark is-rounded is-medium"><span>{"Register"}</span>
                        <span class="icon is-small">
                           <i class="fas fa-arrow-right"></i>
                        </span>
                     </button>
                   </div>
            }

        }
    }
}

#[derive(Serialize)]
struct Registration {
    user: String,
    pass: String,
    email: String
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            registration_success: false,
            name: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            repeat_password: "".to_string()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RegisterEvent => {
                log::info!("Registering user! {} - {}", self.name, self.email);
                let registration = Registration {
                    user: "".to_string(),
                    pass: "".to_string(),
                    email: "".to_string()
                };
                ctx.link().send_future(async move {
                    let resp = Request::post("/server/api/v1/register")
                        .header("Content-Type", "application/json")
                        .body(json!(registration).to_string())
                        .send()
                        .await;

                    match resp {
                        Ok(response) => {
                            match response.status() {
                                200 => {
                                    log::info!("Success");
                                    Msg::RegisteredEvent(true) 
                                },
                                _ => {
                                    log::error!("Error registering user, non OK status");
                                    Msg::RegisteredEvent(false) 
                                }
                            }
                        },
                        Err(e) => {
                            log::error!("Error registering user {:?}", e);
                            Msg::RegisteredEvent(false) 
                        }
                    }

                });

            }
            Msg::InputEvent(val, typ) => {
                match typ {
                    Name => self.name = val,
                    Email => self.email = val,
                    Password => self.password = val,
                    RepeatPassword => self.repeat_password = val
                }
            }
            Msg::RegisteredEvent(val) => {
                log::info!("Registered event!");
                if val {
                    self.registration_success = true;
                }
            }
        };
        true
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
                                          { self.register_card(ctx)}
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
