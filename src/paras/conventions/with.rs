use super::Has;

pub trait With<T>
where
    Self: Clone + Has<T>,
{
    /// with will put a <T> in the relevant place(s).
    #[must_use]
    fn with(&self, x: T) -> Self;

    /// with_fn will apply a mapping function to the <T>s in relevant places,
    /// but may skip Option<T>.
    #[must_use]
    fn with_fn(&self, f: fn(T) -> T) -> Self {
        if let Some(x) = self.get().map(f) {
            return self.with(x);
        }
        self.clone()
    }

    /// with_default will put a mapped <T> into the relevant place(s),
    /// inserting a default value if the actual entry was missing.
    #[must_use]
    fn with_or(&self, f: fn(T) -> T, def: T) -> Self {
        if let Some(x) = self.get() {
            return self.with(f(x));
        }
        self.with(def)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_default_impl() {
        let x: Option<i32> = None;
        assert_eq!(Some(1), x.with(1));
        assert_eq!(None, x.with_fn(|x: i32| x + 1));
        assert_eq!(Some(5), x.with_or(|x| x + 1, 5));
    }
}
