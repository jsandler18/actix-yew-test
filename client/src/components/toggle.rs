use yew::prelude::*;
use context::Context;

pub struct ToggleComponent {
    on: bool,
    on_string: String,
    off_string: String,
}

pub enum Msg {
    Toggle,
}

#[derive(PartialEq, Clone)]
pub struct Properties {
    pub on_string: String,
    pub off_string: String,
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            on_string: String::from(""),
            off_string: String::from(""),
        }
    }
}

impl Component<Context> for ToggleComponent {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, _env: &mut Env<Context, Self>) -> Self {
        ToggleComponent {
            on: false,
            on_string: props.on_string,
            off_string: props.off_string,
        }
    }

    fn update(&mut self, msg: Self::Message, _env: &mut Env<Context,Self>) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.on = !self.on;
                true
            }
        }
    }
}

impl Renderable<Context, ToggleComponent> for ToggleComponent {
    fn view(&self) -> Html<Context, Self> {
        html! {
            // Render your model here
            <button onclick=|_| Msg::Toggle,>{ if self.on { &self.on_string } else { &self.off_string } }</button>
        }
    }
}