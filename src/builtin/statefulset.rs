use k8s_openapi::api::{apps::v1::StatefulSet, core::v1::PodTemplateSpec};

use crate::paras::conventions::Has;

impl Has<PodTemplateSpec> for StatefulSet {
    fn get(&self) -> Option<PodTemplateSpec> {
        self.spec.clone().map(|x| x.template)
    }
}
