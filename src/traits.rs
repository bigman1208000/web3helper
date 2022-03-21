pub trait GetAddress {
    fn get_address(&self, feed_name: &str) -> Option<String>;
}
