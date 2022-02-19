use yew::{Component, Context, Html};
use yew::prelude::*;

pub struct Home;

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
            <div class="container">
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
                            <button class="button is-primary">{"Start your free trial"}</button>
                            <button class="button">{"Documentation"}</button>
                        </div>
                    </div>
                </div>
            </div>
            <div class="container">
                <div class="columns is-centered">
                    <div class="column">
                        <div class="card m-3 my-6">
                          <div class="card-image">
                            <figure class="image is-4by3">
                              <img />
                            </figure>
                          </div>
                          <div class="card-content">
                              <div class="media-content">
                                <p class="title is-4">{"Markdown"}</p>
                                <p class="subtitle is-6">{"Concise and familiar syntax. Take notes with ease"}</p>
                              </div>
                            <br/>
                            <div class="content">
                              {"Lorem ipsum leo risus, porta ac consectetur ac, vestibulum at eros. Donec id elit non mi porta gravida at eget metus. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Cras mattis consectetur purus sit amet fermentum."}
                            </div>
                          </div>
                        </div>
                    </div>
                    <div class="column">
                        <div class="card m-3 my-6">
                          <div class="card-image">
                            <figure class="image is-4by3">
                              <img />
                            </figure>
                          </div>
                          <div class="card-content">
                              <div class="media-content">
                                <p class="title is-4">{"Cross platform"}</p>
                                <p class="subtitle is-6">{"Mobile, desktop, it doesn't matter"}</p>
                              </div>
                            <br/>
                            <div class="content">
                              {"Lorem ipsum leo risus, porta ac consectetur ac, vestibulum at eros. Donec id elit non mi porta gravida at eget metus. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Cras mattis consectetur purus sit amet fermentum."}
                            </div>
                          </div>
                        </div>

                    </div>

                    <div class="column">
                        <div class="card m-3 my-6">
                          <div class="card-image">
                            <figure class="image is-4by3">
                              <img />
                            </figure>
                          </div>
                          <div class="card-content">
                              <div class="media-content">
                                <p class="title is-4">{"Cloud Sync"}</p>
                                <p class="subtitle is-6">{"Start at your desk, end on the couch. Everything is synced."}</p>
                              </div>
                                <br/>
                              <div class="content">
                              {"Lorem ipsum leo risus, porta ac consectetur ac, vestibulum at eros. Donec id elit non mi porta gravida at eget metus. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Cras mattis consectetur purus sit amet fermentum."}
                              </div>
                          </div>
                        </div>
                    </div>
                </div>
            </div>
            </>
        }
    }
}