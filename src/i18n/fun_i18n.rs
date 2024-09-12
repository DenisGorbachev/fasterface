use derive_getters::Getters;
use derive_more::{From, Into};
use standard_traits::Contains;
use std::sync::LazyLock;

#[derive(Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct FunI18n {
    /// Technical function name in CamelCase (e.g. `"CreateRustWorkspace"`)
    name: String,
    /// Human-readable function name (e.g. `"Create a Rust workspace"`)
    title: String,
    /// Human-readable description (e.g. `"Create a Rust project with a workspace structure"`)
    description: String,
    /// Notes (may contain implementation details) (e.g. `vec!["Returns an error if the directory is not empty"]`)
    notes: Vec<String>,
}

pub static FUN_I18N_ALL_EN: LazyLock<Vec<FunI18n>> = LazyLock::new(FunI18n::en_all);

impl FunI18n {
    pub fn new(name: impl Into<String>, title: impl Into<String>, description: impl Into<String>, notes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            name: name.into(),
            title: title.into(),
            description: description.into(),
            notes: notes.into_iter().map(Into::into).collect(),
        }
    }

    /// This is a temporary function (the content will be moved to a separate file in the future)
    pub fn en_all() -> Vec<Self> {
        let new = Self::new;
        vec![
            new(
                "GetFilesInDirectory",
                "Get files in a directory",
                "",
                vec![
                    "Returns only filenames, not full paths",
                    "Does not return . and ..",
                    "Includes hidden files",
                ],
            ),
            new("GetGithubIssues", "Get GitHub issues for a specific repository", "", vec!["Returns all issues, including closed ones"]),
            new("SendAnthropicMessage", "Send a new request to Anthropic Messages API", "", vec!["Uses streaming"]),
            new("AddRustDependency", "Add a new dependency to the Cargo.toml", "", vec!["If the specified Cargo.toml defines a package in a workspace, it adds a dependency specification to both workspace-level Cargo.toml and package-level Cargo.toml"]),
        ]
    }
}

impl Contains<str> for FunI18n {
    fn contains(&self, needle: &str) -> bool {
        self.name.contains(needle) || self.title.contains(needle) || self.description.contains(needle)
    }
}
