#[macro_use]
extern crate yew;

pub mod components;

use yew::prelude::*;
use components::toggle::Toggle;

struct Model {
}

enum Msg {
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            // Render your model here
            <Toggle: on_string="on", off_string="off", />
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}