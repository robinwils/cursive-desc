use cursive::views::{ BoxedView, ScreensView };
use quick_xml::Reader;
use quick_xml::events::{ BytesStart, Event };
use quick_xml::events::attributes::Attribute;
use cursive::views::*;
use std::io::{ BufRead };
use crate::error::Error;
use crate::utils::*;

pub struct Parser<B: BufRead> {
    root_view: ScreensView<BoxedView>,
    reader: Reader<B>,
}

impl<B: BufRead> Parser<B> {
    pub fn new(reader: Reader<B>) -> Parser<B> {
        Parser{
            root_view: ScreensView::new(),
            reader: reader,
        }
    }

    pub fn string_attributes(&self, attr: &Attribute) -> (String, String) {
        let key = String::from_utf8(attr.key.to_vec()).expect("cannot convert key to string");
        let value = attr.unescape_and_decode_value(&self.reader).expect("cannot unescape value");

        (key, value)
    }

    fn add_text_view(&mut self, tag: &BytesStart) -> Result<(), Error> {
        println!("Generating {}", String::from_utf8(tag.name().to_vec()).unwrap());
        let mut text_view = TextView::empty();

        for attr_res in tag.attributes() {
            let attr = attr_res.expect("Malformed attribute");
            let (key, value) = self.string_attributes(&attr);

            match key.as_ref() {
                "content" => text_view.set_content(value),
                "effect" => text_view.set_effect(effect_from_str(value.as_str()).expect("cannot convert to effect")),
                _ => return Err(Error::UnknownAttr),
            };
        }
        self.root_view.add_screen(BoxedView::boxed(text_view));
        Ok(())
    }

    fn add_text_area(&mut self, tag: &BytesStart) -> Result<(), Error> {
        println!("Generating {}", String::from_utf8(tag.name().to_vec()).unwrap());

        Ok(())
    }

    fn add_linear_layout(&mut self, tag: &BytesStart) -> Result<(), Error> {
        println!("Generating {}", String::from_utf8(tag.name().to_vec()).unwrap());

        Ok(())
    }

    pub fn parse(mut self) -> Result<ScreensView, Error> {
        let mut buf = Vec::new();
        loop {
            match self.reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    println!("START {:?}", String::from_utf8(e.name().to_vec()));
                    match e.name() {
                        b"TextView" => self.add_text_view(e).expect("cannot create TextView"),
                        b"TextArea" => self.add_text_area(e).expect("cannot create TextArea"),
                        _ => (),
                    }
                },
                Ok(Event::Text(e)) => println!("TEXT: {:?}", e),
                Err(e) => panic!("Error at position {}: {:?}", self.reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }
        Ok(self.root_view)
    }
}
