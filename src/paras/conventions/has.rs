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

impl<A, B: Has<A>> HasMany<A> for B {
    fn get_all(&self) -> Option<Vec<A>> {
        self.get().map(|x| vec![x])
    }
}
