use cursive::Cursive;

pub trait CallbackRegistry {
    fn new() -> Self;
    fn get(&self, callback_name: &str) -> fn(&Self, &mut Cursive);
}
