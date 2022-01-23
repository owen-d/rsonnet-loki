use crate::paras::conventions::{Has, With};
use k8s_openapi::api::core::v1::{PodSpec, Volume, VolumeMount};

pub type Volumes = Vec<Volume>;
pub type VolumeMounts = Vec<VolumeMount>;

impl<T> Has<Volumes> for T
where
    T: Has<PodSpec>,
{
    fn get(&self) -> Option<Volumes> {
        self.get().and_then(|x| x.get())
    }
}

impl<T> With<Volumes> for T
where
    T: Clone + With<PodSpec>,
{
    fn with(&self, x: Volumes) -> Self {
        self.with(PodSpec {
            volumes: Some(x),
            ..self.get().unwrap_or_default()
        })
    }
}
