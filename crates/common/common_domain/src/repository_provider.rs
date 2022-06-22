pub trait RepositoryProvider<'a, T> {
    fn get_repository(&'a mut self) -> T;
}
