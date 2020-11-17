use cursive::Cursive;
use cursive_callback::CallbackRegistry;
use cursive_callback_macro::*;
use std::collections::HashMap;

struct ExampleCallbackRegistry {}

#[cursive_callbacks]
impl ExampleCallbackRegistry {
    fn say_hello(&self, cursive: &mut Cursive) {
        println!("Hello World!");
    }

    fn quit(&self, cursive: &mut Cursive) {
        cursive.quit();
    }
}

fn main() {
    let mut cb_registry = ExampleCallbackRegistry::new();
}
