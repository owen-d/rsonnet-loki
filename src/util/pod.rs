use super::conventions::{Has, HasMut};
use k8s_openapi::api::apps::v1::DeploymentSpec;
use k8s_openapi::api::core::v1::{PodSpec, PodTemplateSpec};

impl Has<PodTemplateSpec> for DeploymentSpec {
    fn get(&self) -> Option<PodTemplateSpec> {
        Some(self.template.clone())
    }
}

impl HasMut<PodTemplateSpec> for DeploymentSpec {
    fn with(&self, x: PodTemplateSpec) -> Self {
        DeploymentSpec {
            template: x,
            ..self.clone()
        }
    }
}

impl<T> Has<PodSpec> for T
where
    T: Has<PodTemplateSpec>,
{
    fn get(&self) -> Option<PodSpec> {
        self.get().and_then(|x| x.spec.clone())
    }
}

impl<T> HasMut<PodSpec> for T
where
    T: HasMut<PodTemplateSpec>,
{
    fn with(&self, x: PodSpec) -> Self {
        self.with(PodTemplateSpec {
            spec: Some(x),
            ..self.get().clone().unwrap_or_default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_spec_has_pod_spec() {
        let ps = PodSpec {
            active_deadline_seconds: Some(1),
            ..Default::default()
        };

        let def: DeploymentSpec = Default::default();
        let dep = def.with(ps.clone());
        assert_eq!(
            Some(1),
            dep.get().and_then(|x: PodSpec| x.active_deadline_seconds),
        )
    }
}
