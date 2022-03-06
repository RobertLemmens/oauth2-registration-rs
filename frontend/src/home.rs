use yew::{Component, Context, Html};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

pub struct Home;

impl Home {

    fn card(&self, ctx: &Context<Self>, img_src: &'static str, title: &str, subtitle: &str, text: &str) -> Html {
        html! {
                <div class="card m-3 my-6">
                  <div class="card-image">
                    <figure class="image ">
                      <img src={img_src} />
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
            <div class="banner-wrapper">
                <div class="container">
                    <div class="columns is-centered">
                        <div class="column has-text-centered">
                            <figure class="image has-image-centered">
                              <img style="width:400px; margin-left:auto; margin-right:auto;" src="assets/notes_logo.png"/>
                            </figure>
                            <div class="title">
                                <img />
                                <h1>{"Markdown note taking without slowing down"}</h1>
                            </div>
                            <div class="subtitle">
                                <p>{"Your one-stop shop for markdown note taking. Fast, clean, and available on all major platforms with device cloud sync."}</p>
                            </div>
                            <div class="action-buttons">
                                <button onclick={register_callback} class="button is-primary">{"Start your free trial"}</button>
                                <button class="button">{"Documentation"}</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn features(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="feature-wrapper">
                <div class="container">
                    <div class="columns is-centered">
                        <div class="column has-text-centered">
                            <div class="title has-text-light is-size-2">
                                <img />
                                <h1>{"Features"}</h1>
                            </div>
                        </div>
                    </div>
                    <div class="columns is-centered">
                        <div class="column">
                        {self.card(ctx, "assets/markdown_text3.png", "Markdown", "Concise and familiar syntax. Take notes with ease", "We've designed the app around markdown. A simple and concise format that developers are already familiar with. We don't try to be a WYSIWYG editor, and by keeping the focus on markdown we can keep improving on it and make sure we do it right.")}
                        </div>
                        <div class="column">
                        {self.card(ctx, "assets/crossplatform.png", "Cross platform", "Mobile, desktop, it doesn't matter", "Both desktop and mobile are first class citizens. A note taking app should be able to have a fast workflow from idea to input for on the spot thinking like taking a note in the store. But also accommodate long note taking sessions like lectures or drafts.")}
                        </div>
                        <div class="column">
                        {self.card(ctx, "assets/cloud.png", "Cloud Sync", "Start at your desk, end on the couch. Everything is synced.", "Your notes are automatically synced between devices. You don't even have to think about it. Not only is it a seamless experience, we also take great care of your data. Everything is encrypted by default. Don't like cloud functionality? No problem, you can opt out and keep your notes on your device only.")}
                        </div>
                    </div>

                    <div class="columns is-centered">
                        <div class="column">
                        {self.card(ctx, "assets/performant.png", "Performant", "Starts fast, operates smooth.", "When you use an app you want it to feel fast and snappy. From startup to selecting menu items and typing. We try to keep it all very fast. ")}
                        </div>
                        <div class="column">
                        {self.card(ctx, "assets/easy.png", "Easy", "No complicated fluff.", "No complex UI elements. Search, filter, create new notes and type. It doesn't have to be complex. By keeping it simple, you can focus on writing.")}
                        </div>
                        <div class="column">
                        {self.card(ctx, "assets/shortkeys.png", "Shortkeys", "Use keybindings to reduce downtime.", "Use shortcuts to keep your hand on the keyboard. Create new notes, notebooks, switch settings and UI modes without having to resort back to your mouse.")}
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn showcase(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="showcase-wrapper">
                <div class="container">
                    <figure class="image is-16by9">
                        <img class="" src="assets/notes-app2.png" />
            <p></p>
                    </figure>
                </div>
            </div>
        }
    }

    fn editor_styles(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="styles-wrapper">
                <div class="container">
                    <div class="columns is-centered">
                        <div class="column has-text-centered">
                            <div class="title has-text-light is-size-2">
                                <img />
                                <h1>{"Choose your style"}</h1>
                            </div>
                        </div>
                    </div>
                    <div class="columns is-centered">
                        <div class="column has-text-centered">
                            <figure class="image has-image-centered">
                                <img style="width:800px; margin-left:auto; margin-right:auto;" src="assets/styles.png" />
                            </figure>
                        </div>
                    </div>
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
                {self.banner(ctx)}
                {self.features(ctx)}
                {self.showcase(ctx)}
                {self.editor_styles(ctx)}
            </>
        }
    }
}
