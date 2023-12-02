pub struct FoldMap<'a, I: Iterator + ?Sized, T, O> {
    underlying: &'a mut I,
    state: T,
    function: fn(&mut T, I::Item) -> Option<O>,
}

impl<I: Iterator + ?Sized, T, O> Iterator for FoldMap<'_, I, T, O> {
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.underlying.next()?;
            if let Some(output) = (self.function)(&mut self.state, item) {
                return Some(output);
            }
        }
    }
}

pub trait FoldMapExt<I: Iterator + ?Sized, T, O>: Iterator {
    fn fold_map(
        &mut self,
        state: T,
        function: fn(&mut T, Self::Item) -> Option<O>,
    ) -> FoldMap<Self, T, O> {
        FoldMap {
            underlying: self,
            state,
            function,
        }
    }
}

impl<I: Iterator + ?Sized, T, O> FoldMapExt<I, T, O> for I {}
