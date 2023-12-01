pub struct FoldMap<I, T, O, F>
where
    I: Iterator,
    F: FnMut(&mut T, I::Item) -> Option<O>,
{
    underlying: I,
    storage: T,
    function: F,
}

impl<I, T, O, F> Iterator for FoldMap<I, T, O, F>
where
    I: Iterator,
    F: FnMut(&mut T, I::Item) -> Option<O>,
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(item) = self.underlying.next() {
                if let Some(output) = self.function.call_mut((&mut self.storage, item)) {
                    return Some(output);
                }
            } else {
                return None;
            }
        }
    }
}

pub trait FoldMapExt<I, T, O, F>: Iterator
where
    Self: Sized,
    I: Iterator,
    F: FnMut(&mut T, <Self as Iterator>::Item) -> Option<O>,
{
    fn fold_map(self, state: T, function: F) -> FoldMap<Self, T, O, F> {
        FoldMap {
            underlying: self,
            storage: state,
            function,
        }
    }
}

impl<I, T, O, F> FoldMapExt<I, T, O, F> for I
where
    I: Iterator,
    F: FnMut(&mut T, <Self as Iterator>::Item) -> Option<O>,
{
}
