pub struct FuncIter<T, F: FnMut() -> Option<T>> {
    f: F,
}

impl<T, F: FnMut() -> Option<T>> FuncIter<T, F> {
    pub fn new(f: F) -> FuncIter<T, F> {
        FuncIter { f }
    }
}

impl<T, F: FnMut() -> Option<T>> Iterator for FuncIter<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        (self.f)()
    }
}
