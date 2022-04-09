//! Misc Extensions



pub trait ExtensionMutate<R>
where Self: Sized
{
    fn mutate(&self, f: impl Fn(&Self) -> R) -> R;
}
impl<T, R> ExtensionMutate<R> for T {
    fn mutate(&self, f: impl Fn(&Self) -> R) -> R {
        f(self)
    }
}



#[cfg(test)]
mod tests {
    use crate::utils::extensions::misc::ExtensionMutate;
    
    #[test]
    fn mutate() {
        {
            assert_eq!(
                42_usize,
                vec![-1_i32, 42_i32, 8_i32, 9_i32].iter()
                    .max().unwrap()
                    .mutate(|&x| x as usize)
            );
        }
    }
}

