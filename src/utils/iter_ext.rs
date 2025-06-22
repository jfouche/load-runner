pub trait IterExt: Iterator {
    fn single(mut self) -> Result<Self::Item, SingleIterError>
    where
        Self: Sized,
    {
        match self.next() {
            Some(first) => match self.next() {
                Some(_) => Err(SingleIterError::MoreThanOneResult),
                None => Ok(first),
            },
            None => Err(SingleIterError::NoResult),
        }
    }
}

impl<T> IterExt for T where T: Iterator + ?Sized {}

#[derive(Clone, Copy, Debug)]
pub enum SingleIterError {
    NoResult,
    MoreThanOneResult,
}

impl std::error::Error for SingleIterError {}

impl std::fmt::Display for SingleIterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
