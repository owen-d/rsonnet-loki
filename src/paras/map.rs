#[macro_export]
macro_rules! map {
    (@expand $val: pat$(,)?) => {$val};
    // match no trailing commas
    (@expand $val: pat, $cons: path) => {
        map!(@expand $cons($val))
    };

    ($f: expr, $($cons: path),*) => {
        {
            use $crate::paras::resource::Object;
            let f = move |o: Object| {
                if let map!(@expand val, $($cons),*) = o {
                    return $f(val).into()
                }
                o
            };
            Box::new(f)
        }
    };
    ($f: ident, $($cons: path),*) => {
        {
            use $crate::paras::resource::Object;
            let f = move |o: Object| {
                if let map!(@expand val, $($cons),*) = o {
                    return $f(val).into()
                }
                o
            };
            Box::new(f)
        }
    };
}

#[cfg(test)]
mod tests {
    use k8s_openapi::api::core::v1::Container;

    #[test]
    fn test_map_macro_gen() {
        let f = |mut c: Container| {
            c.image = Some("grafana/loki:main".to_string());
            c
        };

        let _m = map!(f, Object::Container);
    }
}
