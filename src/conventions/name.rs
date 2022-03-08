use derive_more::{From, Into};

// Name corresponds to the labelset ("name", <component>),
// which is used to determine the component. This is then
// used to to do things such bootstrap as anti-affinity rules.
#[derive(PartialEq, From, Into, Clone, Default, Debug)]
pub struct Name(pub String);

impl Name {
    pub fn new(x: String) -> Self {
        Self(x)
    }

    pub fn key() -> String {
        "name".to_string()
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
