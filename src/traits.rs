pub trait HasTitle {
    fn title(&self) -> &str;
}
pub trait HasID {
    fn id(&self) -> u32;
}