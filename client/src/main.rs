#[macro_use]
extern crate yew;

use yew::prelude::*;

struct Model {
    is_clicked: bool
}

enum Msg {
    DoIt,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            is_clicked: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                self.is_clicked = true;
                // Update your model on events
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            // Render your model here
            <button onclick=|_| Msg::DoIt,>{ if self.is_clicked { "clicked" } else {"click me"} }</button>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}