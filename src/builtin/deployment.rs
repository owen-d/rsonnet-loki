use k8s_openapi::api::{
    apps::v1::{Deployment, DeploymentSpec},
    core::v1::PodTemplateSpec,
};

use crate::paras::conventions::{Has, With};

impl Has<PodTemplateSpec> for Deployment {
    fn get(&self) -> Option<PodTemplateSpec> {
        self.spec.clone().map(|x| x.template)
    }
}

impl With<PodTemplateSpec> for Deployment {
    fn with(&self, x: PodTemplateSpec) -> Self {
        Deployment {
            spec: self.spec.clone().map(|s: DeploymentSpec| DeploymentSpec {
                template: x,
                ..s.clone()
            }),
            ..self.clone()
        }
    }
}
