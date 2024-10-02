use std::path::Path;
use walkdir::{DirEntry, WalkDir};

/// Find all `walkdir::DirEntry` that contain `needle` in their name
///
/// `needle` is expected to be in lowercase
pub fn find_entries_by_needle<'a>(path: &'a Path, needle: &'a str) -> impl Iterator<Item = DirEntry> + 'a {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(move |entry| {
            entry
                .file_name()
                .to_str()
                .map(|name| name.to_lowercase().contains(needle))
                .unwrap_or(false)
        })
}
