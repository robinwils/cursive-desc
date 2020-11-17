use crate::error::Error;
use crate::utils::*;
use cursive::direction::Orientation;
use cursive::views::*;
use cursive::View;
use cursive_callback::CallbackRegistry;
use serde::*;
use std::cell::RefCell;
use std::rc::Rc;

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
        callback: String,
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

pub struct Parser<M: CallbackRegistry> {
    cb_reg: Rc<RefCell<M>>,
}

impl<M: 'static> Parser<M>
where
    M: CallbackRegistry,
{
    pub fn new(callback_registry: M) -> Self {
        Self {
            cb_reg: Rc::new(RefCell::new(callback_registry)),
        }
    }

    fn new_text_view(&self, content: String, effect: Option<String>) -> Result<BoxedView, Error> {
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
        &self,
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
            llayout.add_child(
                self.from_jview(child)
                    .expect("cannot parse linear layout children"),
            );
        }

        Ok(BoxedView::boxed(llayout))
    }

    fn new_button(&self, label: String, callback: String) -> Result<BoxedView, Error> {
        let reg = self.cb_reg.to_owned();
        let callback = reg.borrow().get(&callback);

        // Button::new requires its second arg to implement Fn<&mut Cursive> + 'static thus
        // the compiler wants the elements used inside the closure must have a 'static lifetime.
        // What this really means is that the closure needs to own all the elements inside of it,
        Ok(BoxedView::boxed(Button::new(label, move |s| {
            callback(&reg.borrow(), s)
        })))
    }

    pub fn from_jview(&self, jview: JView) -> Result<BoxedView, Error> {
        match jview {
            JView::TextView { content, effect } => self.new_text_view(content, effect),
            JView::LinearLayout {
                orientation,
                children,
            } => self.new_linear_layout(orientation, children),
            JView::Button { label, callback } => self.new_button(label, callback),
        }
    }

    pub fn parse(self, s: &str) -> Result<BoxedView, Error> {
        let doc: JView = serde_json::from_str(s)?;

        println!("Json deserialized: {:?}", doc);

        self.from_jview(doc)
    }
}
