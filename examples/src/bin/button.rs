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
            "view": "LinearLayout",
            "orientation": "vertical",
            "children": [
                {
                    "view": "TextView",
                    "content": "Hello World"
                },
                {
                    "view": "LinearLayout",
                    "orientation": "horizontal",
                    "children": [
                        {
                            "view": "Button",
                            "label": "QUIT",
                            "callback": "quit"
                        },
                        {
                            "view": "Button",
                            "label": "Logs",
                            "callback": "log_hello"
                        }
                    ]
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
