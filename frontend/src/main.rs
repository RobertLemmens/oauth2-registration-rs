mod register;
mod home;
mod login;

use yew::prelude::*;
use yew::html::Scope;
use yew_router::prelude::*;

use home::Home;
use register::Register;
use login::Login;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let onclick = link.callback(|_| Msg::AddOne);
        html! {
            <div>
                <button {onclick}>{ "+1" }</button>
                <p>{ self.value }</p>
                <p>{"topkek"}</p>
            </div>
        }
    }
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/register")]
    Register,

    #[at("/login")]
    Login
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            return html! { <Home /> }
        }
        Route::Register => {
            return html! { <Register /> }
        }
        Route::Login => {
            return html! { <Login />}
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
