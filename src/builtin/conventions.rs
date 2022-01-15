pub trait Has<T> {
    fn get(&self) -> Option<T>;
}

impl<T: Clone> Has<T> for T {
    fn get(&self) -> Option<T> {
        Some(self.clone())
    }
}

impl<T: Clone> Has<T> for Option<T> {
    fn get(&self) -> Self {
        self.clone()
    }
}

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