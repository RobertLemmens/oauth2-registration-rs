use yew::{Component, Context, Html};
use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

use crate::navigation::NavBar;

pub struct Pricing;

impl Pricing {
    fn checkmark(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg class="h-6 w-6 mr-2" xmlns="http://www.w3.org/2000/svg" width="24" height="24" stroke="currentColor" fill="#98C379" viewBox="0 0 1792 1792">
                <path d="M1412 734q0-28-18-46l-91-90q-19-19-45-19t-45 19l-408 407-226-226q-19-19-45-19t-45 19l-91 90q-18 18-18 46 0 27 18 45l362 362q19 19 45 19 27 0 46-19l543-543q18-18 18-45zm252 162q0 209-103 385.5t-279.5 279.5-385.5 103-385.5-103-279.5-279.5-103-385.5 103-385.5 279.5-279.5 385.5-103 385.5 103 279.5 279.5 103 385.5z">
                </path>
            </svg>
        }
    }
}

impl Component for Pricing {
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
            <>
                <NavBar />
                <div class="pricing-wrapper">
                    <div class="container" style="margin-top: 100px;">
                        <div class="columns">
                            <div class="column">
                            </div>
                            <div class="column">
                                <article class="message is-info">
                                  <div class="message-header">
                                    <p>{"Currently: Free alpha"}</p>
                                  </div>
                                  <div class="message-body">
                                    {"While we are in alpha, notes is"}
                                    <strong>{" free "}</strong>
                                    {"to use. Once we enter beta with a 1.0 release candidate we will enter paid. All notes created during the free period will still be accessible (cloud & locally) and alpha users will receive a discount option for a full year."}
                                    <br />
                                    <br />
                                    {"To get started, click "}
                                    <Link<Route> to={Route::Register}> {"Sign up"} </Link<Route>>
                                    {" at the top right in the navbar."}
                                  </div>
                                </article>
                            </div>
                            <div class="column">
                            </div>
                        </div>
                        <div class="columns">
                            <div class="column">

                            </div>
                            <div class="column has-text-centered">

                                <div class="box has-background-white">
                                    <p class="has-text-grey-dark is-size-4 font-medium mb-2">
                                        {"It's simple, one paid plan"}
                                    </p>
                                    <p class="has-text-grey-dark is-size-2 has-text-weight-bold">
                                        {"$3.99"}
                                        <span class="has-text-grey-light text-gray-300 is-size-6">
                                            {"/ month"}
                                        </span>
                                    </p>
                                    <p class="has-text-grey-dark is-size-7 mt-2">
                                        {"Get everything we have to offer."}
                                    </p>
                                    <ul class="has-text-grey-dark is-size-6 mt-3 mb-6">
                                        <li class="mb-3 is-flex is-align-items-center ">
                                            {self.checkmark(ctx)}
                                            {"App on all major platforms"}
                                        </li>
                                        <li class="mb-3 is-flex is-align-items-center ">
                                            {self.checkmark(ctx)}
                                            {"Secure cloud sync"}
                                        </li>
                                        <li class="mb-3 is-flex is-align-items-center ">
                                            {self.checkmark(ctx)}
                                            {"Unlimited notes"}
                                        </li>
                                        <li class="mb-3 is-flex is-align-items-center ">
                                            {self.checkmark(ctx)}
                                            {"Support"}
                                        </li>
                                        <li class="mb-3 is-flex is-align-items-center ">
                                            {self.checkmark(ctx)}
                                            {"A major productivity boost"}
                                        </li>
                                    </ul>
                                    <div class="has-text-centered mt-3" style="display:none;">
                                        <button class="button is-primary " >
                                            {"Free while alpha"}
                                        </button>
                                    </div>
                                </div>

                            </div>
                            <div class="column">
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}