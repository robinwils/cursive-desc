use cursive_xml::*;

pub fn main() {
    let mut siv = cursive::default();
    
    siv.add_layer(cursive_xml::from_str("
    <TextView content='Hello World'></TextView>
    ").unwrap());

    // Starts the event loop.
    siv.run();
}
