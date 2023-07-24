use yew::{html, Component, Context, Html};

mod components;
use components::layout::System;


pub struct App;

impl Component for App {
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
            <main>
             <System />
            </main>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
