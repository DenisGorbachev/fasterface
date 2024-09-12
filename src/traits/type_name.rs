/// A unique identifier of the type
pub trait TypeName {
    fn type_name() -> &'static str;
}
