pub trait Desugar<T> {
    fn desugar(self) -> Option<Vec<T>>;
}
