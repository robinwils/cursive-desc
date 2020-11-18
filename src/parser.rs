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
pub struct JTextView {
    content: String,
    effect: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct JLinearLayout {
    orientation: JOrientation,
    children: Vec<JView>,
}

#[derive(Deserialize, Debug)]
pub struct JButton {
    label: String,
    callback: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "view")]
pub enum JView {
    TextView(JTextView),
    LinearLayout(JLinearLayout),
    Button(JButton),
}

#[derive(Deserialize, Debug, Copy, Clone)]
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

    fn new_text_view(&self, tview: &JTextView) -> Result<BoxedView, Error> {
        println!("Creating TextView: {}, {:?}", tview.content, tview.effect);
        let mut text_view = TextView::empty();

        text_view.set_content(tview.content.to_string());
        if let Some(estring) = &tview.effect {
            text_view
                .set_effect(effect_from_str(estring.as_str()).expect("cannot convert to effect"));
        }

        Ok(BoxedView::boxed(text_view))
    }

    fn new_linear_layout(&self, jlayout: &JLinearLayout) -> Result<BoxedView, Error> {
        println!(
            "Creating LinearLayout: {:?}, {}",
            jlayout.orientation,
            jlayout.children.len()
        );
        let mut llayout = LinearLayout::new(jlayout.orientation.into());

        for child in &jlayout.children {
            llayout.add_child(
                self.from_jview(&child)
                    .expect("cannot parse linear layout children"),
            );
        }

        Ok(BoxedView::boxed(llayout))
    }

    fn new_button(&self, button: &JButton) -> Result<BoxedView, Error> {
        let reg = self.cb_reg.to_owned();
        let callback = reg.borrow().get(&button.callback);

        // Button::new requires its second arg to implement Fn<&mut Cursive> + 'static thus
        // the compiler wants the elements used inside the closure must have a 'static lifetime.
        // What this really means is that the closure needs to own all the elements inside of it,
        Ok(BoxedView::boxed(Button::new(
            button.label.to_string(),
            move |s| callback(&reg.borrow(), s),
        )))
    }

    }

    pub fn from_jview(&self, jview: &JView) -> Result<BoxedView, Error> {
        match jview {
            JView::TextView(tview) => self.new_text_view(tview),
            JView::LinearLayout(llayout) => self.new_linear_layout(llayout),
            JView::Button(button) => self.new_button(button),
        }
    }

    pub fn parse(self, s: &str) -> Result<BoxedView, Error> {
        let doc: JView = serde_json::from_str(s)?;

        println!("Json deserialized: {:?}", doc);

        self.from_jview(&doc)
    }
}
