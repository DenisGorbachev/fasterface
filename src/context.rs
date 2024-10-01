use derive_more::{From, Into};
use derive_new::new;
use rustc_hash::FxHashMap;
use std::any::{Any, TypeId};

#[derive(new, From, Into, Default, Debug)]
pub struct Context {
    map: FxHashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn insert<T: 'static>(&mut self, value: T) -> Option<Box<dyn Any>> {
        let type_id = TypeId::of::<T>();
        self.map.insert(type_id, Box::new(value))
    }

    pub fn insert_by_ref<T: Clone + 'static>(&mut self, value: &T) -> Option<Box<dyn Any>> {
        self.insert(value.clone())
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.map.get(&type_id).map(|v| v.downcast_ref().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use subtype::newtype_path_buf;

    newtype_path_buf!(pub struct Directory(PathBuf));
    newtype_path_buf!(pub struct File(PathBuf));

    macro_rules! assert_eq_local {
        ($left:expr, $right:expr $(,)?) => {
            assert_eq!($left, Some(&$right))
        };
    }

    #[test]
    fn must_store_different_types() {
        let current_directory = Directory::new("/home/alice");
        let current_file = File::new("/home/alice/photo.jpg");
        let mut context = Context::default();
        context.insert(current_directory.clone());
        context.insert(current_file.clone());
        assert_eq_local!(context.get::<Directory>(), current_directory);
        assert_eq_local!(context.get::<File>(), current_file);
    }

    #[test]
    fn must_overwrite_single_type() {
        let old_directory = Directory::new("/home/alice");
        let new_directory = Directory::new("/home/alice/workspace");
        let mut context = Context::default();
        context.insert(old_directory.clone());
        context.insert(new_directory.clone());
        assert_eq_local!(context.get::<Directory>(), new_directory);
    }
}
