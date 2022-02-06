// A trait roughly equivalent to `If let <destructured> = x`
pub trait Matches<A> {
    fn matches(&self) -> Option<A>;
}

impl<A: Clone> Matches<A> for A {
    fn matches(&self) -> Option<A> {
        Some(self.clone())
    }
}

#[macro_export]
macro_rules! impl_matches {
    (@expand $val: pat$(,)?) => {$val};
    // match no trailing commas
    (@expand $val: pat, $cons: path) => {
        impl_matches!(@expand $cons($val),)
    };
    (@expand $val: pat, $cons: path, $($rest: path),*) => {
        impl_matches!(@expand $cons($val), $($rest,)*)
    };

    ($a: ty, $b: ty, $($cons: path),*) => {
        impl $crate::paras::matches::Matches<$a> for $b {
            fn matches(&self) -> Option<$a> {
                if let impl_matches!(@expand val, $($cons),*) = self {
                    return Some(val.clone());
                }
                None
            }
        }
    };
}
