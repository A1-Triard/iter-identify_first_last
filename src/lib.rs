#![no_std]

use core::fmt::{self, Debug, Formatter};
use core::iter::{FusedIterator, Iterator, Map, Peekable};
use core::mem::replace;

pub struct IdentifyFirstLast<I: Iterator>(
    Map<IdentifyFirst<IdentifyLast<I>>, fn(((I::Item, bool), bool)) -> (I::Item, bool, bool)>
);

impl<I: Iterator + Debug> Debug for IdentifyFirstLast<I> where I::Item: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> { self.0.fmt(f) }
}

impl<I: Iterator + Clone> Clone for IdentifyFirstLast<I> where I::Item: Clone {
    fn clone(&self) -> Self { IdentifyFirstLast(self.0.clone()) }
}

impl<I: Iterator> Iterator for IdentifyFirstLast<I> {
    type Item = (I::Item, bool, bool);

    fn next(&mut self) -> Option<Self::Item> { self.0.next() }

    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}

impl<I: ExactSizeIterator> ExactSizeIterator for IdentifyFirstLast<I> {
    fn len(&self) -> usize { self.0.len() }
}

impl<I: FusedIterator> FusedIterator for IdentifyFirstLast<I> { }

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
    fn identify_first_last(self) -> IdentifyFirstLast<Self>;
}

impl<I: Iterator + Sized> IteratorIdentifyFirstLastExt for I {
    fn identify_first(self) -> IdentifyFirst<Self> { IdentifyFirst { is_first: true, iter: self } }

    fn identify_last(self) -> IdentifyLast<Self> { IdentifyLast { iter: self.peekable() } }

    fn identify_first_last(self) -> IdentifyFirstLast<Self> {
        IdentifyFirstLast(self.identify_last().identify_first().map(
            |((item, is_last), is_first)| (item, is_first, is_last)
        ))
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn it_works() -> bool {
        let result = 2 + 2;
        result == 4
    }
}
