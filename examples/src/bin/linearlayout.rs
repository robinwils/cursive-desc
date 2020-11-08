use cursive::Cursive;
use cursive_callback::CallbackRegistry;
use cursive_callback_macro::*;
use cursive_desc::from_str;

struct EmptyCallbackRegistry {}

#[cursive_callbacks]
impl EmptyCallbackRegistry {}

fn main() {
    let data = r#"
        {
            "view": "LinearLayout",
            "orientation": "vertical",
            "children": [
                {
                    "view": "TextView",
                    "content": "Hello"
                },
                {
                    "view": "TextView",
                    "content": "World!"
                }
            ]
        }"#;

    let mut siv = cursive::default();

    let cb_reg = EmptyCallbackRegistry::new();

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    siv.add_layer(from_str(data, EmptyCallbackRegistry::new()).expect("cannot create view"));

    // Run the event loop
    siv.run();
}
