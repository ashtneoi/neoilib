#[cfg(test)]
mod tests;

pub fn func_iter<T, F>(f: F) -> FuncIter<T, F>
where
    F: FnMut() -> Option<T>,
{
    FuncIter::new(f)
}

pub struct FuncIter<T, F>
where
    F: FnMut() -> Option<T>,
{
    f: F,
}

impl<T, F> FuncIter<T, F>
where
    F: FnMut() -> Option<T>,
{
    fn new(f: F) -> FuncIter<T, F> {
        FuncIter { f }
    }
}

impl<T, F> Iterator for FuncIter<T, F>
where
    F: FnMut() -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        (self.f)()
    }
}
