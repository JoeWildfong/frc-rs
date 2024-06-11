#[cfg(feature = "maven")]
pub mod maven;
#[cfg(feature = "from_source")]
pub mod source;

#[derive(PartialEq, Eq, Debug)]
pub enum ArtifactType {
    Static,
    Shared
}

pub struct Build {
    pub maven_name: &'static str,
    pub version: &'static str,
    pub base_name: &'static str,
    pub srcs: Vec<&'static str>,
    pub include: &'static str,
    pub include_env_vars: &'static [&'static str],
}

impl Build {
    #[allow(unused_variables)]
    pub fn build(&self, artifact_type: ArtifactType) {
        #[cfg(feature = "maven")]
        {
            maven::build(self, artifact_type);
            return;
        }
        #[cfg(feature = "from_source")]
        {
            if artifact_type == ArtifactType::Static {
                source::build(self);
                return;
            }
        }
        panic!("no building method enabled - ensure at least one of the maven and from_source features are enabled");
    }
}