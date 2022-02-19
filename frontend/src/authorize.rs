use yew::{Component, Context, Html, html};

pub struct Authorize;

impl Component for Authorize {
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
            <div class="authorize-wrapper"></div>
        }
    }
}