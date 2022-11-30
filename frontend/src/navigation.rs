use yew::{function_component, html, Html};
use yew_router::prelude::*;
use crate::Route;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
            <nav class="navbar is-transparent is-fixed-top">
                <div class="navbar-brand">
                    <a class="navbar-item">
                        <img src="assets/notes_logo.png" height="50"/>
                    </a>
                </div>
                <div class="navbar-menu">
                    <div class="navbar-start">
                        <Link<Route> to={Route::Home} classes="navbar-item">
                            {"Home"}
                        </Link<Route>>

                        <a class="navbar-item">
                            {"About"}
                        </a>
                    </div>
                    <div class="navbar-end">
                        <div class="navbar-item">
                            <div class="buttons">
                                <Link<Route> to={Route::Register} classes="button is-primary">
                                    <strong>{"Sign up"}</strong>
                                </Link<Route>>

                                <Link<Route> to={Route::Login} classes="button">
                                    {"Log in"}
                                </Link<Route>>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
}
