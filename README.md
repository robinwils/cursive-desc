# cursive-desc
JSON description frontend for the cursive library

Look at the examples on how to use

```
use cursive::Cursive;
use cursive_desc::from_str;

fn main() {
    let data = r#"
        {
            "view": "TextView",
            "content": "Hello World!",
            "effect": "bold"
        }"#;

    let mut siv = cursive::default();

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    siv.add_layer(from_str(data).expect("cannot create view"));

    // Run the event loop
    siv.run();
```
