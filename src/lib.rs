use cursive::views::ScreensView;
use cursive::View;

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

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub fn from_str(s: &str) -> Result<impl View, error::Error> {
    let parser: parser::Parser = parser::Parser::new();

    let main_view = parser.parse(s);

    main_view
}
