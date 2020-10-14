use cursive::theme::Effect;
use crate::error::Error;

pub fn effect_from_str(s: &str) -> Result<Effect, Error> {
    match s {
        "simple" => Ok(Effect::Simple),
        "reverse" => Ok(Effect::Reverse),
        "bold" => Ok(Effect::Bold),
        "italic" => Ok(Effect::Italic),
        "strikethrough" => Ok(Effect::Strikethrough),
        "underline" => Ok(Effect::Underline),
        _ => Err(Error::ParseStrError),
    }
}
