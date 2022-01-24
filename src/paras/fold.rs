use anyhow::Result;

pub trait Foldable<T>
where
    Self: Sized,
{
    fn fold(self, f: fn(T) -> T) -> Result<Self>;
}

#[macro_export]
macro_rules! unexpected_type {
    ($t: ty) => {
        bail!(
            "unexpected type enum variant, wanted {}",
            std::any::type_name::<$t>()
        );
    };
}

// #[macro_export]
// macro_rules! impl_fold {
//     ($constructor:ident) => {

//         use anyhow::bail;
//         use $crate::paras::resource::Object;
//         use $crate::unexpected_type;
//         if let Object::$constructor(x) = f(self.into()) {
//             return Ok(x);
//         }
//         unexpected_type!(Self);
//     };
// }

#[macro_export]
macro_rules! impl_fold {
    ($t: ty, $cons: ident) => {
        impl $crate::paras::fold::Foldable<$crate::paras::resource::Object> for $t {
            fn fold(
                self,
                f: fn($crate::paras::resource::Object) -> $crate::paras::resource::Object,
            ) -> anyhow::Result<Self> {
                if let $crate::paras::resource::Object::$cons(x) = f(self.into()) {
                    return Ok(x);
                }
                crate::unexpected_type!($t);
            }
        }
    };
}
