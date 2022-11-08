macro_rules! redirect {
    ($url:expr) => {
        rocket::response::Redirect::to(uri!($url))
    };
}
pub(crate) use redirect;
