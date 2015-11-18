//! Ngram Set
//!
//! Data structure that holds a number of items, and uses n-grams of
//! of the items to enable fuzzy matching and comparison operations

#![deny(missing_docs,
       missing_debug_implementations, missing_copy_implementations,
       trivial_casts, trivial_numeric_casts,
       unsafe_code,
       unstable_features,
       unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(feature = "dev", deny(clippy))]

extern crate ngrams;

use std::iter::FromIterator;
use std::convert::AsRef;
use std::hash;
use std::collections::{HashMap, HashSet};

/// Primary data structure, consolidates the set and the map of ngrams -> items
#[derive(Debug, Clone)]
pub struct NgramSet<'a, T: AsRef<str> + Clone + PartialEq + Eq + hash::Hash> {
    items: HashSet<T>,
    grams: HashMap<&'a str, Vec<T>>,
}

impl<'a, T: AsRef<str> + Clone + PartialEq + Eq + hash::Hash> NgramSet<'a, T> {
    /// Primary way to construct an `NgramSet`
    pub fn new<U: IntoIterator<Item=T>>(items: U) -> NgramSet<'a, T> {
        NgramSet {
          items: HashSet::<T>::from_iter(items),
          grams: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
}
