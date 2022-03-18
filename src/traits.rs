pub(crate) trait GetAddress {
    fn get_address(&self) -> &'static str;
}
