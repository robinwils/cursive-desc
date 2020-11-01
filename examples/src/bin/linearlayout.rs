use cursive::Cursive;
use cursive_desc::from_str;

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

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    siv.add_layer(from_str(data).expect("cannot create view"));

    // Run the event loop
    siv.run();
}
