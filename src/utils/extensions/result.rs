//! `Result` extensions



pub trait AliasResultIntoOption<T> {
    fn into_option(self) -> Option<T> where Self: Sized;
}
impl<T, E> AliasResultIntoOption<T> for Result<T, E> {
    #[inline]
    fn into_option(self) -> Option<T> {
        self.ok()
    }
}
