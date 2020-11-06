use cursive_callback::CallbackMap;
use cursive_callback_macro::Callback;
use std::collections::HashMap;

#[derive(Callback)]
struct ExampleCallbackMap {
    #[cbmap]
    callbacks: HashMap<String, String>,
}

impl ExampleCallbackMap {
    fn say_hello() {
        println!("Hello World!");
    }
}

fn main() {}
