use anyhow::Result;

pub trait Foldable<T>
where
    Self: Sized,
{
    fn fold(self, f: fn(T) -> T) -> Result<Self>;
}

impl<A, B> Foldable<A> for Option<B>
where
    B: Foldable<A>,
{
    fn fold(self, f: fn(A) -> A) -> Result<Self> {
        match self.map(|x| x.fold(f)) {
            Some(res) => res.map(Some),
            None => Ok(None),
        }
    }
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
/// Arguments take the form (<implementing_type>, <constructor_name>, [<subfield>...])
/// where `implementing_type` is the target type for this implementation,
/// `constructor_name` is the Object:: constructor (by convention the same name as `implementing_type`)
/// and a list of `subfields` that should also be folded. Subfields ensure that the fold
/// can be recursively applied into subfields that implement it, for instance mapping
/// DeploymentSpec -> PodTemplateSpec -> PodSpec
///                \                  \
///                 \> ObjectMeta      \> ObjectMeta
///
/// For instance, `impl_fold!(PodTemplateSpec, PodTemplateSpec, metadata, spec)`
/// will generate the `Foldable<Object> for PodTemplateSpec` by also descending
/// into the `metadata` and `spec` fields of `PodTemplateSpec` and `fold`ing there as well.
/// NB: folds are applied in a depth first order, meaning `fold(PodTemplateSpec)` in this case
/// folds the subfields `metadata` and `spec` first, then applies the mapping function to the
/// resulting `PodTemplateSpec`.
#[macro_export]
macro_rules! impl_fold {
    ($t: ty, $cons: ident, $( $field: ident ),*) => {
        impl $crate::paras::fold::Foldable<$crate::paras::resource::Object> for $t {
            fn fold(
                self,
                f: fn($crate::paras::resource::Object) -> $crate::paras::resource::Object,
            ) -> anyhow::Result<Self> {
                let x = Self {
                    $(
                        $field: self.$field.fold(f)?,
                    )*
                        ..self
                };
                if let $crate::paras::resource::Object::$cons(val) = f(x.into()) {
                    return Ok(val);
                }
                crate::unexpected_type!($t);
            }
        }
    };
}
