use super::Has;

pub trait With<T>
where
    Self: Clone + Has<T>,
{
    #[must_use]
    fn with(&self, x: T) -> Self;
}

impl<T> With<T> for T
where
    T: Clone,
{
    fn with(&self, x: T) -> Self {
        x
    }
}

impl<T> With<T> for Option<T>
where
    T: Clone,
{
    fn with(&self, x: T) -> Self {
        Some(x)
    }
}
