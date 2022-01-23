use k8s_openapi::{apimachinery::pkg::apis::meta::v1::ObjectMeta, api::core::v1::Service};

use crate::paras::conventions::Has;

impl Has<ObjectMeta> for Service {
    fn get(&self) -> Option<ObjectMeta> {
        Some(self.metadata.clone())
    }
}
