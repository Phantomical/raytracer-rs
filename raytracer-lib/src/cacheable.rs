pub trait Cacheable<T>: Send + Sync {
    fn cached(&self) -> T;
}

impl<T: Clone + Send + Sync> Cacheable<T> for T {
    fn cached(&self) -> T {
        self.clone()
    }
}
