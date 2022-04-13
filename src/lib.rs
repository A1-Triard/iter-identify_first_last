#![no_std]

use core::fmt::{self, Debug, Formatter};
use core::iter::{FusedIterator, Iterator, Peekable};
use core::mem::replace;

pub struct IdentifyLast<I: Iterator> {
    iter: Peekable<I>,
}

impl<I: Iterator + Debug> Debug for IdentifyLast<I> where I::Item: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> { self.iter.fmt(f) }
}

impl<I: Iterator + Clone> Clone for IdentifyLast<I> where I::Item: Clone {
    fn clone(&self) -> Self { IdentifyLast { iter: self.iter.clone() } }
}

impl<I: Iterator> Iterator for IdentifyLast<I> {
    type Item = (I::Item, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            let is_last = self.iter.peek().is_none();
            Some((item, is_last))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<I: ExactSizeIterator> ExactSizeIterator for IdentifyLast<I> {
    fn len(&self) -> usize { self.iter.len() }
}

impl<I: FusedIterator> FusedIterator for IdentifyLast<I> { }

#[derive(Debug, Clone)]
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
    fn identify_last(self) -> IdentifyLast<Self>;
}

impl<I: Iterator + Sized> IteratorIdentifyFirstLastExt for I {
    fn identify_first(self) -> IdentifyFirst<Self> { IdentifyFirst { is_first: true, iter: self } }
    fn identify_last(self) -> IdentifyLast<Self> { IdentifyLast { iter: self.peekable() } }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
