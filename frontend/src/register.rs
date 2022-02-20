use yew::{Component, Context, Html};
use yew::prelude::*;

pub struct Register;

impl Register {
    fn register_card(&self, ctx: &Context<Self>) -> Html{
        html! {
                   <div class="box">
                    <h1 class="title is-2 note-fg-dark">{"Register"}</h1>
                     <div class="field">
                         <div class="control has-icons-left">
                             <input class="input is-medium" placeholder="Email address" type="text" />
                             <span class="icon is-small is-left">
                               <i class="fas fa-envelope"></i>
                             </span>
                         </div>
                     </div>
                     <div class="field">
                         <div class="control has-icons-left">
                             <input class="input is-medium" placeholder="Full Name" type="text" />
                             <span class="icon is-small is-left">
                               <i class="fas fa-user"></i>
                             </span>
                         </div>
                     </div>
                     <div class="field">
                         <div class="control has-icons-left">
                             <input class="input is-medium" placeholder="Password" type="password" />
                             <span class="icon is-small is-left">
                               <i class="fas fa-lock"></i>
                             </span>
                         </div>
                     </div>
                     <div class="field">
                         <div class="control has-icons-left">
                             <input class="input is-medium" placeholder="Repeat password" type="password" />
                             <span class="icon is-small is-left">
                               <i class="fas fa-lock"></i>
                             </span>
                         </div>
                     </div>
                     <button class="button notes-bg-dark is-rounded is-medium"><span>{"Register"}</span>
                        <span class="icon is-small">
                           <i class="fas fa-arrow-right"></i>
                        </span>
                     </button>
                   </div>
        }
    }
}

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
