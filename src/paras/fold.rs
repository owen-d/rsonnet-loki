use anyhow::Result;

use super::matches::Matches;

// Foldable is a trait for an entity which can be folded into `Result<Self>` via
// a function with signature `Fn(T) -> Result<T>`.
// So far, `<T>` has exclusively been `Self`.
pub trait Foldable<T>
where
    Self: Sized,
{
    fn fold(self, f: &dyn Fn(T) -> Result<T>) -> Result<Self>;
}

impl<A, B> Foldable<A> for Option<B>
where
    B: Foldable<A>,
{
    fn fold(self, f: &dyn Fn(A) -> Result<A>) -> Result<Self> {
        match self.map(|x| x.fold(f)) {
            Some(res) => res.map(Some),
            None => Ok(None),
        }
    }
}

impl<A, B> Foldable<A> for Vec<B>
where
    B: Foldable<A>,
{
    fn fold(self, f: &dyn Fn(A) -> Result<A>) -> Result<Self> {
        self.into_iter().map(|x| x.fold(f)).collect()
    }
}

// Folder is a trait for the function-like entity which
// _performs_ the fold. It's mainly used for playing type-tetris
// to ensure we can define and group together (this is existential typing)
// a set of disparate functions into compatible folds, for instance
// folding a resource with a mapping function which only consumes statefulsets
// and folding a resource with a mapping function which only consumes containers.
// These can both be expressed in the same `Vec<Box<dyn Folder<Object>>>`.
pub trait Folder<A> {
    fn apply(&self, x: A) -> Result<A>;
}

// This is the main implementation we'll be using. It turns
// a mapping function over a variant of the type and turns
// it into a fold over the entire type!
impl<A, B> Folder<B> for Box<dyn Fn(A) -> A>
where
    B: From<A> + Foldable<B> + Matches<A>,
{
    fn apply(&self, x: B) -> Result<B> {
        foldmap(self, x)
    }
}

// Ugh i have no idea what to call this, but it's not the same
// as the `foldmap` you're probably expecting :(
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
        impl $crate::paras::fold::Foldable<$crate::paras::resource::Object> for $t {
            fn fold(
                self,
                f: &dyn Fn($crate::paras::resource::Object) -> anyhow::Result<$crate::paras::resource::Object>,
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
