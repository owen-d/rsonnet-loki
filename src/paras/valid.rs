use anyhow::Result;

use super::{fold::Foldable, matches::Matches};

pub trait Validator<A: Clone> {
    fn validate(&self, x: &A) -> Result<()>;
}

impl<A, B> Validator<B> for Box<dyn Fn(&A) -> Result<()>>
where
    B: Matches<A> + Foldable<B, B, B> + Clone,
{
    fn validate(&self, x: &B) -> Result<()> {
        x.clone()
            .fold(&|v: B| {
                if let Some(val) = v.matches() {
                    let _ = self(&val)?;
                }
                Ok(v)
            })
            .map(|_| ())
    }
}
