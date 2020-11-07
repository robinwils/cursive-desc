use cursive::Cursive;
use cursive_callback::CallbackMap;
use cursive_callback_macro::*;
use std::collections::HashMap;

#[derive(Callback)]
struct ExampleCallbackMap {
    #[cbmap]
    callbacks: HashMap<String, fn(&mut Cursive)>,
}

#[cursive_callbacks]
impl ExampleCallbackMap {
    fn say_hello(cursive: &mut Cursive) {
        println!("Hello World!");
    }

    fn say(cursive: &mut Cursive) {
        println!("IN SAY");
    }
}

fn main() {
    let callback_map = ExampleCallbackMap::new();

    println!("CB MAP = {:?}", callback_map.callbacks.keys());
}
