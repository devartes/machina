use yew::prelude::*;

pub struct System;

impl Component for System {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <section class="flex justify-center items-center w-full h-screen bg-black">
                <p class="font-black text-9xl text-yellow-400 text-center">{"Template Tailwind"}</p>
            </section>
        }
    }
}