use anyhow::Result;

use super::fold::Foldable;

pub trait Matches<A> {
    fn matches(&self) -> Option<A>;
}

pub fn foldmap<A, B>(f: &dyn Fn(A) -> A, x: B) -> Result<B>
where
    B: From<A> + Foldable<B> + Matches<A>,
{
    return x.fold(&|v: B| {
        if let Some(val) = v.matches() {
            return Ok(f(val).into());
        }
        Ok(v)
    });
}

#[macro_export]
macro_rules! impl_matches {
    (@expand $val: pat$(,)?) => {$val};
    // match no trailing commas
    (@expand $val: pat, $cons: path) => {
        impl_fold!(@expand $cons($val),)
    };
    (@expand $val: pat, $cons: path, $($rest: path),*) => {
        impl_fold!(@expand $cons($val), $($rest,)*)
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