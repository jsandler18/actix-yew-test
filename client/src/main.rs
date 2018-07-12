extern crate yew;

use yew::prelude::*;

fn main() {
    println!("Hello, world!");
    yew::initialize();

    //
    // App::<RootComponent>::new().mount_to_body();

    yew::run_loop();
}
