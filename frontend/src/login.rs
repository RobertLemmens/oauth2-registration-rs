use yew::prelude::*;

pub struct Login;

impl Component for Login {
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
                    <div class="column is-three-quarters">
                        <div class="columns is-centered">
                            <div class="column is-half">
                                <div class="login-form">


                                    <section class="hero is-fullheight">
                                      <div class="hero-body has-text-centered">
                                        <div class="container">
                                          <div class="box">
                                            <div class="field">
                                                <label class="label">{"Email"}</label>
                                                <div class="control has-icons-left">
                                                    <input class="input is-medium" placeholder="Email address" type="text" />
                                                    <span class="icon is-small is-left">
                                                      <i class="fas fa-user"></i>
                                                    </span>
                                                </div>
                                            </div>
                                            <div class="field">
                                                <label class="label">{"Password"}</label>
                                                <div class="control has-icons-left">
                                                    <input class="input is-medium" placeholder="Password" type="password" />
                                                    <span class="icon is-small is-left">
                                                      <i class="fas fa-lock"></i>
                                                    </span>
                                                </div>
                                            </div>
                                          </div>
                                        </div>
                                      </div>
                                    </section>


                                </div>
                            </div>
                       </div> 
                    </div>
                    <div class="column">
                    </div>
                </div>
            </div>
        }
    }
}
