use yew::{Component, Context, Html};
use yew::prelude::*;
use yew_router::prelude::*;
use yew::functional::*;

use crate::Route;

pub struct Home;

impl Home {

    fn card(&self, ctx: &Context<Self>, title: &str, subtitle: &str, text: &str) -> Html {
        html! {
                <div class="card m-3 my-6">
                  <div class="card-image">
                    <figure class="image is-4by3">
                      <img />
                    </figure>
                  </div>
                  <div class="card-content">
                      <div class="media-content">
                        <p class="title is-4">{title}</p>
                        <p class="subtitle is-6">{subtitle}</p>
                      </div>
                    <br/>
                    <div class="content">
                      {text}
                    </div>
                  </div>
                </div>
        }
    }

    fn banner(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let history: AnyHistory = link.history().unwrap();
        let register_callback = Callback::from(move |_| history.push(Route::Register));
        html! {
            <div class="columns is-centered">
                <div class="column has-text-centered">
                    <figure class="image has-image-centered">
                      <img style="width:400px; margin-left:auto; margin-right:auto;" src="assets/notes_logo.png"/>
                    </figure>
                    <div class="title">
                        <img />
                        <h1>{"Simple, Fast, Perfect."}</h1>
                    </div>
                    <div class="subtitle">
                        <p>{"Your one-stop shop for markdown note taking. Available on all major platforms and syncs to all your devices."}</p>
                    </div>
                    <div class="action-buttons">
                        <button onclick={register_callback} class="button is-primary">{"Start your free trial"}</button>
                        <button class="button">{"Documentation"}</button>
                    </div>
                </div>
            </div>
        }
    }

    fn features(&self, ctx: &Context<Self>) -> Html {
        html! {
                <div class="columns is-centered">
                    <div class="column">
                    {self.card(ctx, "Markdown", "Concise and familiar syntax. Take notes with ease", "Lorem ipsum leo risus, porta ac consectetur ac, vestibulum at eros. Donec id elit non mi porta gravida at eget metus. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Cras mattis consectetur purus sit amet fermentum.")}
                    </div>
                    <div class="column">
                    {self.card(ctx, "Cross platform", "Mobile, desktop, it doesn't matter", "Lorem ipsum leo risus, porta ac consectetur ac, vestibulum at eros. Donec id elit non mi porta gravida at eget metus. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Cras mattis consectetur purus sit amet fermentum.")}
                    </div>
                    <div class="column">
                    {self.card(ctx, "Cloud Sync", "Start at your desk, end on the couch. Everything is synced.", "Lorem ipsum leo risus, porta ac consectetur ac, vestibulum at eros. Donec id elit non mi porta gravida at eget metus. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Cras mattis consectetur purus sit amet fermentum.")}
                    </div>
                </div>
        }
    }

}

impl Component for Home {
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
            <>
            <div class="container top-banner">
                {self.banner(ctx)}
            </div>
            <div class="container">
                {self.features(ctx)}
            </div>
            </>
        }
    }
}
