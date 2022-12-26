#![cfg_attr(feature="nightly", feature(trusted_len))]

#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

//! ## Feature flags
#![doc=document_features::document_features!()]

#![no_std]

#[doc=include_str!("../README.md")]
type _DocTestReadme = ();

use core::fmt::{self, Debug, Formatter};
use core::iter::{FusedIterator, Iterator, Map, Peekable};
#[cfg(feature="nightly")]
use core::iter::TrustedLen;
use core::mem::replace;

/// An iterator that yields the marking the boundary elements boolean flags and the element during iteration.
///
/// This struct is created by the [`identify_first_last`](IteratorIdentifyFirstLastExt::identify_first_last)
/// extension method on [`Iterator`]. See its documentation for more.
pub struct IdentifyFirstLast<I: Iterator>(
    Map<IdentifyFirst<IdentifyLast<I>>, fn((bool, (bool, I::Item))) -> (bool, bool, I::Item)>
);

impl<I: Iterator + Debug> Debug for IdentifyFirstLast<I> where I::Item: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> { self.0.fmt(f) }
}

impl<I: Iterator + Clone> Clone for IdentifyFirstLast<I> where I::Item: Clone {
    fn clone(&self) -> Self { IdentifyFirstLast(self.0.clone()) }
}

impl<I: Iterator> Iterator for IdentifyFirstLast<I> {
    type Item = (bool, bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> { self.0.next() }

    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}

#[cfg(feature="nightly")]
unsafe impl<I: TrustedLen> TrustedLen for IdentifyFirstLast<I> { }

impl<I: ExactSizeIterator> ExactSizeIterator for IdentifyFirstLast<I> {
    fn len(&self) -> usize { self.0.len() }
}

impl<I: FusedIterator> FusedIterator for IdentifyFirstLast<I> { }

/// An iterator that yields the marking the last element boolean flag and the element during iteration.
///
/// This struct is created by the [`identify_last`](IteratorIdentifyFirstLastExt::identify_last)
/// extension method on [`Iterator`]. See its documentation for more.
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
    type Item = (bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            let is_last = self.iter.peek().is_none();
            Some((is_last, item))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

#[cfg(feature="nightly")]
unsafe impl<I: TrustedLen> TrustedLen for IdentifyLast<I> { }

impl<I: ExactSizeIterator> ExactSizeIterator for IdentifyLast<I> {
    fn len(&self) -> usize { self.iter.len() }
}

/// An iterator that yields the marking the first element boolean flag and the element during iteration.
///
/// This struct is created by the [`identify_first`](IteratorIdentifyFirstLastExt::identify_first)
/// extension method on [`Iterator`]. See its documentation for more.
#[derive(Debug, Clone)]
pub struct IdentifyFirst<I: Iterator> {
    is_first: bool,
    iter: I,
}

impl<I: Iterator> Iterator for IdentifyFirst<I> {
    type Item = (bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            let is_first = replace(&mut self.is_first, false);
            Some((is_first, item))
        } else {
            self.is_first = true;
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

#[cfg(feature="nightly")]
unsafe impl<I: TrustedLen> TrustedLen for IdentifyFirst<I> { }

impl<I: ExactSizeIterator> ExactSizeIterator for IdentifyFirst<I> {
    fn len(&self) -> usize { self.iter.len() }
}

impl<I: FusedIterator> FusedIterator for IdentifyFirst<I> { }

/// Extends [`Iterator`] with methods for marking first and last elements with boolean flags.
pub trait IteratorIdentifyFirstLastExt: Iterator + Sized {
    /// Creates an iterator which gives the `is_first` flag as well as the next value.
    ///
    /// The iterator returned yields pairs `(is_first, val)`,
    /// where `is_first` is the boolean which is true for the first element and false for the others,
    /// and `val` is the value returned by the iterator.
    fn identify_first(self) -> IdentifyFirst<Self>;

    /// Creates an iterator which gives the `is_last` flag as well as the next value.
    ///
    /// The iterator returned yields pairs `(is_last, val)`,
    /// where `is_last` is the boolean which is true for the last element and false for the others,
    /// and `val` is the value returned by the iterator.
    fn identify_last(self) -> IdentifyLast<Self>;

    /// Creates an iterator which gives the `is_first` and `is_last` flags as well as the next value.
    ///
    /// The iterator returned yields triples `(is_first, is_last, val)`,
    /// where `is_first` is the boolean which is true for the first element and false for the others,
    /// `is_last` is the boolean which is true for the last element and false for the others,
    /// and `val` is the value returned by the iterator.
    fn identify_first_last(self) -> IdentifyFirstLast<Self>;
}

impl<I: Iterator + Sized> IteratorIdentifyFirstLastExt for I {
    fn identify_first(self) -> IdentifyFirst<Self> { IdentifyFirst { is_first: true, iter: self } }

    fn identify_last(self) -> IdentifyLast<Self> { IdentifyLast { iter: self.peekable() } }

    fn identify_first_last(self) -> IdentifyFirstLast<Self> {
        IdentifyFirstLast(self.identify_last().identify_first().map(
            |(is_first, (is_last, item))| (is_first, is_last, item)
        ))
    }
}

#[cfg(test)]
mod tests {
    use arrayvec::ArrayVec;
    use crate::IteratorIdentifyFirstLastExt;

    #[test]
    fn identify_first() {
        assert_eq!(
            [1, 2, 3, 4].into_iter().identify_first().collect::<ArrayVec<_, 4>>().as_slice(),
            &[(true, 1), (false, 2), (false, 3), (false, 4)]
        );
    }

    #[test]
    fn identify_last() {
        assert_eq!(
            [1, 2, 3, 4].into_iter().identify_last().collect::<ArrayVec<_, 4>>().as_slice(),
            &[(false, 1), (false, 2), (false, 3), (true, 4)]
        );
    }

    #[test]
    fn identify_first_last() {
        assert_eq!(
            [1, 2, 3, 4].into_iter().identify_first_last().collect::<ArrayVec<_, 4>>().as_slice(),
            &[(true, false, 1), (false, false, 2), (false, false, 3), (false, true, 4)]
        );
    }
}
