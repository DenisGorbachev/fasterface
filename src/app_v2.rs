use crate::{find_entries_by_needle, Container, Input, LocationV1, Panel, SupportText, Text, Vertical, VirtualWidget};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use not_found_error::NotFoundError;
use stub_macro::stub;
use LocationV1::*;

#[derive(Debug)]
pub struct Entry;

#[derive(Debug)]
pub struct Action;

#[derive(new, Getters, From, Into, Default, Debug)]
pub struct Fasterface {
    location: LocationV1,
    input: String,
    support_text: SupportText,
}

impl Fasterface {
    pub fn set_input(&mut self, input: impl Into<String>) {
        self.input = input.into();
    }

    pub fn run_first_entry(&mut self) -> Result<(), NotFoundError<Entry>> {
        self.location = self.get_first_entry_result()?;
        Ok(())
    }

    pub fn get_first_entry_result(&mut self) -> Result<LocationV1, NotFoundError<Entry>> {
        match &self.location {
            None => Err(NotFoundError::<Entry>::new()),
            ThePathBuf(path_buf) => {
                if let Some(entry) = find_entries_by_needle(path_buf, &self.input.to_lowercase()).next() {
                    return Ok(ThePathBuf(entry.into_path()));
                }
                if let Some(_action) = self.actions().next() {
                    return Ok(stub!(LocationV1));
                }
                Err(NotFoundError::<Entry>::new())
            }
        }
    }

    pub fn actions(&self) -> impl Iterator<Item = Action> {
        stub!(impl dyn Iterator<Item = Action>)
    }

    pub fn to_panel(&self) -> Panel {
        let children: Vec<Box<dyn VirtualWidget>> = vec![
            Box::new(Input::new(self.input.clone())),
            Box::new(Text::new(self.support_text.clone())),
        ];
        let container = Container::new(children, Vertical);
        Panel::new("Fasterface", container)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Outcome;
    use crate::TemporaryFile;

    impl From<&TemporaryFile> for Fasterface {
        fn from(file: &TemporaryFile) -> Self {
            Self {
                location: file.path_buf().clone().into(),
                ..Self::default()
            }
        }
    }

    #[test]
    #[ignore]
    fn must_delete_file() -> Outcome {
        let dummy_name = "dummy";
        let dummy_file = TemporaryFile::create(format!("/{dummy_name}"))?;
        let mut fasterface = Fasterface::from(&dummy_file);
        fasterface.set_input("delete");
        fasterface.run_first_entry()?;
        // TODO: A lot of values are multi-contextual: e.g. a Path can be also a RustProjectRoot, a PythonProjectRoot, a DjangoProjectRoot. The user would want to open the path & have us discover the potential contexts automatically.
        todo!()
    }

    #[test]
    #[ignore]
    fn must_search_for_files() {
        // TODO: It takes time to display results after the input has been set
        // TODO: Some results might be cached
        // TODO: The cache might not be populated yet
        // TODO: Some results may come incrementally (e.g. pages from a paginated API endpoint)
        todo!()
    }

    #[test]
    #[ignore]
    fn must_show_progress_indicator_during_search() {
        todo!()
    }
}
