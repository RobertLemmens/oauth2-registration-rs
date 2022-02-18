use yew::{Component, Context, Html};
use yew::prelude::*;

pub struct Register;

impl Component for Register {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="register-wrapper">
                <p>{"Register page"}</p>
            </div>
        }
    }
}