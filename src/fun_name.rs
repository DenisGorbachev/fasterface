use crate::traits::Fun;
use FunName::*;
use strum::EnumString;

#[derive(EnumString, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum FunName {
    GetFilesInDirectory,
    GetGithubIssues,
    SendAnthropicMessage,
    AddRustDependency,
}

impl FunName {
    pub fn to_fun(self) -> Box<dyn Fun> {
        match self {
            GetFilesInDirectory => {
                todo!()
            }
            GetGithubIssues => todo!(),
            SendAnthropicMessage => todo!(),
            AddRustDependency => todo!(),
        }
    }
}
