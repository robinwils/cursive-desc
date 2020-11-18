use cursive::Cursive;
use cursive_callback::CallbackRegistry;
use cursive_callback_macro::*;
use cursive_desc::from_str;
use log::info;

struct CursiveCallbacks {}

#[cursive_callbacks]
impl CursiveCallbacks {
    fn quit(&self, cursive: &mut Cursive) {
        cursive.quit();
    }

    fn log_hello(&self, cursive: &mut Cursive) {
        info!("Hello!");
        cursive.show_debug_console();
    }
}

fn main() {
    let data = r#"
        {
            "view": "Dialog",
            "content": {
                "view": "TextView",
                "content": "Hello World"
            },
            "title": "Simple",
            "dismiss": "Close",
            "buttons": [
                {
                    "view": "Button",
                    "label": "Console",
                    "callback": "log_hello"
                }
            ]
        }"#;

    let mut siv = cursive::default();

    let cb_reg = CursiveCallbacks::new();

    cursive::logger::init();

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    siv.add_layer(from_str(data, cb_reg).expect("cannot create view"));

    // Run the event loop
    siv.run();
}
