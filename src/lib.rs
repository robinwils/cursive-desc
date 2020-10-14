use quick_xml::Reader;
use cursive::views::ScreensView;

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

mod parser;
mod error;
mod utils;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub fn from_str(s: & str) -> Result<ScreensView, error::Error> {
    let mut reader = Reader::from_str(s);
    reader.trim_text(true);
    let parser: parser::Parser<&[u8]> = parser::Parser::new(reader);

    let screens_view = parser.parse();

    screens_view
}
