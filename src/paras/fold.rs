use anyhow::Result;

use super::matches::Matches;

pub trait Foldable<A, B, C>
where
    Self: Sized,
{
    fn fold(self, f: &dyn Fn(A) -> Result<B>) -> Result<C>;
}

impl<A, B, C, D> Foldable<A, B, Option<C>> for Option<D>
where
    D: Foldable<A, B, C>,
{
    fn fold(self, f: &dyn Fn(A) -> Result<B>) -> Result<Option<C>> {
        match self.map(|x| x.fold(f)) {
            Some(res) => res.map(Some),
            None => Ok(None),
        }
    }
}

impl<A, B, C, D> Foldable<A, B, Vec<C>> for Vec<D>
where
    D: Foldable<A, B, C>,
{
    fn fold(self, f: &dyn Fn(A) -> Result<B>) -> Result<Vec<C>> {
        self.into_iter().map(|x| x.fold(f)).collect()
    }
}

pub trait Folder<A, B> {
    fn apply(&self, x: A) -> Result<B>;
}

// This is the main implementation we'll be using. It turns
// a mapping function over a variant of the type and turns
// it into a fold over the entire type!
impl<A, B, C, D> Folder<C, D> for Box<dyn Fn(A) -> Result<B>>
where
    C: Matches<A> + Foldable<C, D, D>,
    D: From<B> + From<C>,
{
    fn apply(&self, x: C) -> Result<D> {
        x.fold(&|v: C| {
            if let Some(val) = v.matches() {
                return self(val).map(Into::into);
            }
            Ok(v.into())
        })
    }
}

#[macro_export]
macro_rules! impl_from_chain {
    ($src: ty, $interim: ty, $dst: ty) => {
        impl From<$src> for $dst {
            fn from(x: $src) -> $dst {
                let tmp: $interim = x.into();
                tmp.into()
            }
        }
    };
}

#[macro_export]
macro_rules! unexpected_type {
    ($t: ty) => {
        anyhow::bail!(
            "unexpected type enum variant, wanted {}",
            std::any::type_name::<$t>()
        );
    };
}

/// Creates Foldable<Object> implementations for most k8s api objects
/// that are bound by Object:: constructors as part of that enum.
/// Arguments take the form (<implementing_type>, [<subfield>...])
/// where `implementing_type` is the target type for this implementation
/// `subfields` is a list of embedded fields that should also be folded. Subfields
/// ensure that the fold can be recursively applied into subfields that implement it,
/// such as:
///
/// DeploymentSpec -> PodTemplateSpec -> PodSpec
///                \                  \
///                 \> ObjectMeta      \> ObjectMeta
///
/// For instance, `impl_fold!(PodTemplateSpec, metadata, spec)`
/// will generate the `Foldable<Object, Object, Self> for PodTemplateSpec` by also descending
/// into the `metadata` and `spec` fields of `PodTemplateSpec` and `fold`ing there as well.
/// NB: folds are applied in a depth first order, meaning `fold(PodTemplateSpec)` in this case
/// folds the subfields `metadata` and `spec` first, then applies the mapping function to the
/// resulting `PodTemplateSpec`.
#[macro_export]
macro_rules! impl_fold {
    // shortcut for no foldable embedded fields
    ($t: ty) =>{
        impl_fold!($t,);
    };
    ($t: ty, $( $field: ident ),*) => {
        impl $crate::paras::fold::Foldable<$crate::paras::resource::Object, $crate::paras::resource::Object, Self> for $t {
            fn fold(
                self,
                f: &dyn Fn($crate::paras::resource::Object) -> anyhow::Result<$crate::paras::resource::Object>,
            ) -> anyhow::Result<Self> {
                use $crate::paras::matches::Matches;
                let x = Self {
                    $(
                        $field: self.$field.fold(f)?,
                    )*
                        ..self
                };
                f(x.into()).and_then(|val| {
                    let m: Option<Self> = val.matches();
                    match m {
                        Some(v) => Ok(v),
                        None => {
                            crate::unexpected_type!($t);
                        },
                    }
                })
            }
        }
    };

}
