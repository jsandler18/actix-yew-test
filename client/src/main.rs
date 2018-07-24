#[macro_use]
extern crate yew;
extern crate yew_simple;

pub mod components;

use yew::prelude::*;
use components::{
    toggle::ToggleComponent,
    row::RowComponent
};
use yew::services::{
    fetch::FetchService,
    storage::{
        StorageService,
        Area,
    },
    console::ConsoleService,
};

pub mod context {
    use yew::services::{
        fetch::FetchService,
        storage::StorageService,
        console::ConsoleService,
    };

    pub struct Context {
        pub web: FetchService,
        pub store: StorageService,
        pub console: ConsoleService,
    }
}

use context::Context;

struct Model {
    click_count: usize
}

enum Msg {
    Click
}


impl Component<Context> for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _env: &mut Env<Context, Self>) -> Self {
        Model {
            click_count: 0
        }
    }

    fn update(&mut self, msg: Self::Message, _env: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Click => self.click_count += 1
        };
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {

            // Render your model here
            <button onclick=|_| Msg::Click,> {"Click here"} </button>
            <ul><RowComponent: item={&self.click_count},/></ul>
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        web: FetchService::new(),
        store: StorageService::new(Area::Local),
        console: ConsoleService::new(),
    };
    App::<Context, Model>::new(context).mount_to_body();
    yew::run_loop();
}