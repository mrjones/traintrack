use utils;

pub struct EmptyCookieAccessor{ }

impl utils::CookieAccessor for EmptyCookieAccessor {
    fn get_cookie(&self, _key: &str) -> Vec<String> { return vec![]; }
}
