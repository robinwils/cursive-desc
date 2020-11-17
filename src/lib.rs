use cursive::views::BoxedView;
use cursive::View;
use cursive_callback::CallbackRegistry;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn deserialize_text_view() {
        let xml = "<TextView>Test</TextView>";
        let res = from_str(xml);
    }
}

mod error;
pub mod parser;
mod utils;

extern crate serde;

pub fn from_str<M: 'static + CallbackRegistry>(
    s: &str,
    callback_registry: M,
) -> Result<BoxedView, error::Error> {
    let parser = parser::Parser::new(callback_registry);

    parser.parse(s)
}
