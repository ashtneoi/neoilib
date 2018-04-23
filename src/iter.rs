use std::iter::Peekable;

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

pub fn peek_while<'a, I, P>(iter: &'a mut Peekable<I>, predicate: P)
    -> PeekWhile<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    PeekWhile { iter: iter, predicate }
}

pub struct PeekWhile<'a, I, P>
where
    I: 'a + Iterator,
    P: FnMut(&I::Item) -> bool,
{
    iter: &'a mut Peekable<I>,
    predicate: P,
}

impl<'a, I, P> Iterator for PeekWhile<'a, I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let take = match self.iter.peek() {
            Some(ref x) => Some((self.predicate)(x)),
            None => None,
        };
        match take {
            Some(true) => Some(self.iter.next().unwrap()),
            _ => None,
        }
    }
}
