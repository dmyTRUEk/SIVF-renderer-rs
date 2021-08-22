//! `Result` extensions



pub trait TraitAliasResultIntoOption<T> {
    fn into_option(self) -> Option<T> where Self: Sized;
}
impl<T, E> TraitAliasResultIntoOption<T> for Result<T, E> {
    #[inline]
    fn into_option(self) -> Option<T> {
        self.ok()
    }
}
