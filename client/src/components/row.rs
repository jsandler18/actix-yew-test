use yew::prelude::*;
use context::Context;

pub struct RowComponent {
    item: String
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Properties {
    pub item: usize
}

impl Default for Properties {
    fn default() -> Self {
        Properties {
            item: 0,

        }
    }
}

impl Component<Context> for RowComponent {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, _env: &mut Env<Context, Self>) -> Self {
        RowComponent {
            item: props.item.to_string()
        }
    }

    fn update(&mut self, msg: Self::Message, _env: &mut Env<Context, Self>) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties, _env: &mut Env<Context, Self>) -> ShouldRender {
        self.item = props.item.to_string();
        true
    }
}

impl Renderable<Context, RowComponent> for RowComponent {
    fn view(&self) -> Html<Context, Self> {
        html! {
            // Render your model here
            <li> {&self.item} </li>
        }
    }
}