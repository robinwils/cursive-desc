use crate::error::Error;
use crate::utils::*;
use cursive::direction::Orientation;
use cursive::views::*;
use cursive::View;
use serde::*;

#[derive(Deserialize, Debug)]
#[serde(tag = "view")]
pub enum JView {
    TextView {
        content: String,
        effect: Option<String>,
    },
    LinearLayout {
        orientation: JOrientation,
        children: Vec<JView>,
    },
    Button {
        label: String,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum JOrientation {
    Horizontal,
    Vertical,
}

impl Into<Orientation> for JOrientation {
    #[inline]
    fn into(self) -> Orientation {
        match self {
            JOrientation::Horizontal => Orientation::Horizontal,
            JOrientation::Vertical => Orientation::Vertical,
        }
    }
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    fn new_text_view(content: String, effect: Option<String>) -> Result<BoxedView, Error> {
        println!("Creating TextView: {}, {:?}", content, effect);
        let mut text_view = TextView::empty();

        text_view.set_content(content);
        if let Some(estring) = effect {
            text_view
                .set_effect(effect_from_str(estring.as_str()).expect("cannot convert to effect"));
        }

        Ok(BoxedView::boxed(text_view))
    }

    fn new_linear_layout(
        orientation: JOrientation,
        children: Vec<JView>,
    ) -> Result<BoxedView, Error> {
        println!(
            "Creating LinearLayout: {:?}, {}",
            orientation,
            children.len()
        );
        let mut llayout = LinearLayout::new(orientation.into());

        for child in children {
            llayout
                .add_child(Self::from_jview(child).expect("cannot parse linear layout children"));
        }

        Ok(BoxedView::boxed(llayout))
    }

    fn new_button(label: String) -> Result<BoxedView, Error> {
        Ok(BoxedView::boxed(DummyView))
    }

    pub fn from_jview(jview: JView) -> Result<BoxedView, Error> {
        match jview {
            JView::TextView { content, effect } => Self::new_text_view(content, effect),
            JView::LinearLayout {
                orientation,
                children,
            } => Self::new_linear_layout(orientation, children),
            JView::Button { label } => Self::new_button(label),
        }
    }

    pub fn parse(self, s: &str) -> Result<BoxedView, Error> {
        let doc: JView = serde_json::from_str(s)?;

        println!("Json deserialized: {:?}", doc);

        Self::from_jview(doc)
    }
}
