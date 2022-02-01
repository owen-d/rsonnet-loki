use anyhow::Result;

pub trait Foldable<T, F>
where
    Self: Sized,
    F: Fn(T) -> Result<T>,
{
    fn fold(self, f: &F) -> Result<Self>;
}

impl<A, B, F> Foldable<A, F> for Option<B>
where
    B: Foldable<A, F>,
    F: Fn(A) -> Result<A>,
{
    fn fold(self, f: &F) -> Result<Self> {
        match self.map(|x| x.fold(f)) {
            Some(res) => res.map(Some),
            None => Ok(None),
        }
    }
}

impl<A, B, F> Foldable<A, F> for Vec<B>
where
    B: Foldable<A, F>,
    F: Fn(A) -> Result<A>,
{
    fn fold(self, f: &F) -> Result<Self> {
        self.into_iter().map(|x| x.fold(f)).collect()
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
/// A more verbose form can also be specified in the case of nested enum constructors:
/// impl_from_chain!(StatefulSet, Resource, Object); // First ensure From<StatefulSet> for Object
/// impl_fold!(
///     StatefulSet,
///     [Resource::StatefulSet, Object::Resource],
///     metadata,
///     spec
/// );
#[macro_export]
macro_rules! impl_fold {
    (@expand $val: pat$(,)?) => {$val};
    // match no trailing commas
    (@expand $val: pat, $cons: path) => {
        impl_fold!(@expand $cons($val),)
    };
    (@expand $val: pat, $cons: path, $($rest: path),*) => {
        impl_fold!(@expand $cons($val), $($rest,)*)
    };


    // shortcut for no trailing comma and no subfields to be folded.
    ($t: ty, $cons: path) => { impl_fold!($t, $cons,);};
    // shortcut for only one constructor with a comma and possible subfields.
    ($t: ty, $cons: path, $( $field: ident ),*) => {
        impl_fold!($t, [$cons], $($field),*);
    };

    // This is the main macro. It takes a type, a slice of constructors,
    // and an optional comma + optional list of subfields to fold. This is
    // the most verbose form.
    // $(,)? is a shortcut to match a trailing comma or not
    ($t: ty, [$($cons: path),+]$(,)? $( $field: ident ),*) => {
        impl <F: Fn($crate::paras::resource::Object) -> anyhow::Result<$crate::paras::resource::Object>> $crate::paras::fold::Foldable<$crate::paras::resource::Object, F> for $t {
            fn fold(
                self,
                f: &F,
            ) -> anyhow::Result<Self> {
                let x = Self {
                    $(
                        $field: self.$field.fold(f)?,
                    )*
                        ..self
                };
                if let Ok(impl_fold!(@expand val, $($cons),*)) = f(x.into()) {
                    return Ok(val);
                }
                crate::unexpected_type!($t);
            }
        }
    };

}
