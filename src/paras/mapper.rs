use super::conventions::With;

#[macro_export]
macro_rules! map_resource {
	  ( $mapper: expr, $($constructor:ident ),+ ) => {
        {
            use $crate::paras::resource::Resource;
            |r: Resource| {
                $(
                    if let Resource::$constructor(val) = r {
                        return Resource::$constructor(val.with_fn($mapper))
                    }
                )*
                    r
            }

        }
    };
}

#[macro_export]
macro_rules! map_resource_or {
	  ( $mapper: expr, $def: expr,  $($constructor:ident ),+ ) => {
        {
            use $crate::paras::resource::Resource;
            |r: Resource| {
                $(
                    if let Resource::$constructor(val) = r {
                        return Resource::$constructor(val.with_or($mapper, $def))
                    }
                )*
                    r
            }

        }
    };
}

#[cfg(test)]
mod tests {
    use crate::paras::{
        conventions::{Has, Name, With},
        resource::Resource,
    };

    use super::*;
    use k8s_openapi::{
        api::{apps::v1::Deployment, core::v1::PodTemplateSpec},
        apimachinery::pkg::apis::meta::v1::ObjectMeta,
    };

    #[test]
    fn test_map_no_macro() {
        let n = Name::new("foo".to_string());
        let with_name = |x: ObjectMeta| x.with(Name::new("foo".to_string()));
        let pt: PodTemplateSpec = Default::default();

        // First, apply a mapper to None, ensuring None is returned
        let with_altered = pt.with_fn(with_name);
        let exp: Option<ObjectMeta> = None;
        assert_eq!(exp, with_altered.get());

        // Apply a default, ensuring the default exists
        let overridden = with_altered.with_or(|x| x, n.clone());
        assert_eq!(Some(n.clone()), overridden.get());

        // apply a mapper over the inserted default, ensuring it's applied
        let remapped = overridden.with_fn(|_: Name| Name::new("boo".to_string()));
        assert_eq!(Some(Name::new("boo".to_string())), remapped.get())
    }

    #[test]
    fn test_simple_map_resource_macro() {
        let d: Deployment = Default::default();
    }

    #[test]
    fn test_map_resource_macro() {
        let d: Deployment = Default::default();
        let n = Name::new("foo".to_string());
        let x = d.with(n);
        println!("{:#?}", x);
        assert_eq!(Some(Name::new("foo".to_string())), x.get());
        // ensure mapping the resource works
        let m = map_resource!(|_: Name| Name::new("boo".to_string()), Deployment);
        let output = m(Resource::Deployment(x));
        if let Resource::Deployment(mapped) = output {
            println!("{:#?}", mapped);
            assert_eq!(Some(Name::new("boo".to_string())), mapped.get())
        } else {
            panic!("didn't produce deployment type")
        }
    }
}
