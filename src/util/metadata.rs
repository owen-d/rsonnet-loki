// use super::pod::HasPodSpec;
// use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

// pub trait HasMeta {
//     fn metadata(&self) -> Option<ObjectMeta>;
// }

// impl<T: HasPodSpec> HasMeta for T {
//     fn metadata(&self) -> Option<ObjectMeta> {
//         self.pod_spec().and_then(|x| )
//     }
// }
