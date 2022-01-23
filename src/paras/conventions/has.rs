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

pub trait HasMany<T> {
    fn get_all(&self) -> Option<Vec<T>>;
}

impl<A, B> HasMany<A> for Vec<B>
where
    B: Has<A>,
{
    fn get_all(&self) -> Option<Vec<A>> {
        self.into_iter()
            .filter_map(|x| x.get())
            .collect::<Vec<A>>()
            .into()
    }
}
