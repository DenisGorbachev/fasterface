use derive_getters::{Dissolve, Getters};
use derive_new::new;
use semver::Version;
use url::Url;
use url_macro::url;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Package {
    #[new(into)]
    url: Url,
    #[new(into)]
    description: String,
    #[new(into)]
    version: Version,
}

impl Package {
    pub fn all() -> Vec<Self> {
        let new = Self::new;
        vec![new(
            url!("https://jsr.io/@fasterface/find-replace"),
            "Replace each occurrence of input string with replacement string in a specified directory recursively",
            "1.0.0".parse::<Version>().unwrap(),
        )]
    }
}
