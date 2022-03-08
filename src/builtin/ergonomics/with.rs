use super::has::Has;

pub trait With<A> {
    fn with(self, x: A) -> Self;
}

pub trait WithMap<A>: With<A> + Has<A> + Sized {
    fn with_map(self, f: &dyn Fn(A) -> A) -> Self {
        let x = self.has();
        self.with(f(x))
    }
}
