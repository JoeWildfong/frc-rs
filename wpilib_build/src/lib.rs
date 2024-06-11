#[cfg(feature = "maven")]
pub mod maven;
#[cfg(feature = "from_source")]
pub mod source;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Linkage {
    Static,
    Shared,
}

pub struct Build {
    pub maven_url_name: &'static str,
    pub maven_link_name: &'static str,
    pub version: &'static str,
    pub base_name: &'static str,
    pub srcs: Vec<&'static str>,
    pub include: &'static str,
    pub include_env_vars: &'static [&'static str],
}

impl Build {
    #[allow(unused_variables)]
    pub fn build(&self, linkage: Linkage) {
        #[cfg(feature = "maven")]
        {
            let target = std::env::var("TARGET").unwrap();
            if let Some(t) = maven::MavenTarget::from_rustc_target(&target) {
                maven::build(self, linkage, t);
                return;
            } else {
                println!("cargo::warning=could not match target {target} to a prebuilt maven artifact");
            }
        }
        #[cfg(feature = "from_source")]
        {
            if linkage == Linkage::Static {
                source::build(self);
                return;
            } else {
                println!("cargo::warning=from-source builds of shared wpilib artifacts are not supported");
            }
        }

        if cfg!(not(any(feature = "maven", feature = "from_source"))) {
            panic!("no building method enabled - ensure at least one of the maven and from_source features are enabled");
        } else {
            panic!("no enabled building method could be applied");
        }
    }
}
