use crate::error::Error;
use crate::utils::*;
use cursive::views::*;
use cursive::View;
use serde::*;

#[derive(Deserialize, Debug)]
#[serde(tag = "view")]
pub enum JView {
    TextView { content: String, effect: String },
}

pub struct Parser {
    root_view: ScreensView<BoxedView>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            root_view: ScreensView::new(),
        }
    }

    fn new_text_view(content: String, effect: String) -> Result<impl View, Error> {
        let mut text_view = TextView::empty();

        text_view.set_content(content);
        text_view.set_effect(effect_from_str(effect.as_str()).expect("cannot convert to effect"));

        Ok(BoxedView::boxed(text_view))
    }

    pub fn from_jview(jview: JView) -> Result<impl View, Error> {
        match jview {
            JView::TextView { content, effect } => Self::new_text_view(content, effect),
        }
    }

    pub fn parse(mut self, s: &str) -> Result<impl View, Error> {
        let doc: JView = serde_json::from_str(s)?;

        println!("Json deserialized: {:?}", doc);

        Self::from_jview(doc)
    }
}
