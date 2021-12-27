use super::conventions::{Has, HasMut};
use k8s_openapi::api::core::v1::{PodSpec, Volume};

type Volumes = Vec<Volume>;

impl<T> Has<Volumes> for T
where
    T: Has<PodSpec>,
{
    fn get(&self) -> Option<Volumes> {
        self.get().and_then(|x| x.get())
    }
}

impl<T> HasMut<Volumes> for T
where
    T: Clone + HasMut<PodSpec>,
{
    fn with(&self, x: Volumes) -> Self {
        self.with(PodSpec {
            volumes: Some(x),
            ..self.get().unwrap_or_default()
        })
    }
}
