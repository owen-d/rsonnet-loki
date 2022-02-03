use anyhow::Result;

use super::fold::Foldable;

pub trait Matches<A> {
    fn matches(&self) -> Option<A>;
}

pub fn foldmap<A, B>(f: &dyn Fn(A) -> A, x: B) -> Result<B>
where
    B: From<A> + Foldable<B> + Matches<A>,
{
    let m = |v: B| {
        if let Some(val) = v.matches() {
            return Ok(f(val).into());
        }
        Ok(v)
    };
    return x.fold(&m);
}
