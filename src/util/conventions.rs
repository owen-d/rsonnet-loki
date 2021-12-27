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

pub trait HasMut<T>
where
    Self: Clone + Has<T>,
{
    fn with(&self, x: T) -> Self;
}

impl<T> HasMut<T> for T
where
    T: Clone,
{
    fn with(&self, x: T) -> Self {
        x
    }
}

impl<T> HasMut<T> for Option<T>
where
    T: Clone,
{
    fn with(&self, x: T) -> Self {
        Some(x)
    }
}
