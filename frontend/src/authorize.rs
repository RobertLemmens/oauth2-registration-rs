use yew::{Component, Context, Html, html};

pub struct Authorize;

impl Authorize {
    fn authorize_card(&self, ctx: &Context<Self>) -> Html {
        html! {
               <div class="box">
                <h1 class="title is-2 note-fg-dark">{"Authorize Leafpad"}</h1>
                <div class="content">
                    <p class="subtitle is-4">{"Read notes"}</p>
                    <p class="subtitle is-4">{"Write notes"}</p>
                    <p class="subtitle is-4">{"Profile"}</p>
                </div>
                 <button class="button notes-bg-dark is-rounded is-medium"><span>{"Authorize"}</span>
                    <span class="icon is-small">
                       <i class="fas fa-check"></i>
                    </span>
                 </button>
               </div>
        }
    }
}

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