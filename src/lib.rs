#![no_std]

use core::iter::{FusedIterator, Iterator};
use core::mem::replace;

pub struct IdentifyFirst<I: Iterator> {
    is_first: bool,
    iter: I,
}

impl<I: Iterator> Iterator for IdentifyFirst<I> {
    type Item = (I::Item, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            let is_first = replace(&mut self.is_first, false);
            Some((item, is_first))
        } else {
            self.is_first = true;
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<I: ExactSizeIterator> ExactSizeIterator for IdentifyFirst<I> {
    fn len(&self) -> usize { self.iter.len() }
}

impl<I: FusedIterator> FusedIterator for IdentifyFirst<I> { }

pub trait IteratorIdentifyFirstLastExt: Iterator + Sized {
    fn identify_first(self) -> IdentifyFirst<Self>;
}

impl<I: Iterator + Sized> IteratorIdentifyFirstLastExt for I {
    fn identify_first(self) -> IdentifyFirst<Self> { IdentifyFirst { is_first: true, iter: self } }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
