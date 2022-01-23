use k8s_openapi::api::{apps::v1::Deployment, core::v1::PodTemplateSpec};

use crate::paras::conventions::Has;

impl Has<PodTemplateSpec> for Deployment {
    fn get(&self) -> Option<PodTemplateSpec> {
        self.spec.clone().map(|x| x.template)
    }
}
