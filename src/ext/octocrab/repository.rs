use derive_more::{Display, Error, From};

pub fn get_owner_and_name_from_full_name(_full_name: &str) -> Result<(String, String), GetOwnerAndNameFromFullNameError> {
    todo!()
}

#[derive(Error, Display, From, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum GetOwnerAndNameFromFullNameError {}
