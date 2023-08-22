/// Builder trait for building a request
///
/// T: The type of request this builder will build
pub trait Builder<T> {
    fn new() -> Self;
    fn build(&mut self) -> T;
}
