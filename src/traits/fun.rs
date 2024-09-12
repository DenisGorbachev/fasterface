pub trait Fun {
    fn name() -> &'static str
    where
        Self: Sized;
}
