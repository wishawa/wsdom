use std::{collections::HashSet, hash::Hash};

use proc_macro2::{Ident, Span};

pub fn to_snake_case(camel_case: &str) -> String {
    let orig_len = camel_case.len();
    let mut out = String::with_capacity(orig_len + orig_len / 2);
    let mut was_lower = false;
    for ch in camel_case.chars() {
        let is_upper = ch.is_ascii_uppercase();
        let ch_lower = ch.to_ascii_lowercase();
        if was_lower && is_upper {
            out.push('_');
            out.push(ch_lower);
        } else {
            out.push(ch_lower);
        }
        was_lower = !is_upper;
    }
    out
}

pub fn new_ident_safe(name: &str) -> Ident {
    let name = match name {
        "self" => "self_value",
        "type" => "ty",
        n => n,
    };
    let name = name.replace('-', "_");
    Ident::new_raw(&name, Span::call_site())
}

pub fn iter_dedupe_consecutive<I: Iterator, K: Eq>(
    iter: I,
    key: impl FnMut(&I::Item) -> Option<K>,
) -> impl Iterator<Item = I::Item> {
    struct Iter<I: Iterator, C> {
        key: C,
        inside: std::iter::Peekable<I>,
    }
    impl<I: Iterator, K: Eq, C: FnMut(&I::Item) -> Option<K>> Iterator for Iter<I, C> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let item = self.inside.next()?;
                let item_key = (self.key)(&item);
                let next_key = self.inside.peek().and_then(|next| (self.key)(next));
                match (item_key, next_key) {
                    (Some(x1), Some(x2)) if x1 == x2 => continue,
                    _ => return Some(item),
                }
            }
        }
    }
    Iter {
        key,
        inside: iter.peekable(),
    }
}

pub fn iter_dedupe_all<I: Iterator, K: Eq + Hash>(
    iter: I,
    key: impl FnMut(&I::Item) -> Option<K>,
) -> impl Iterator<Item = I::Item> {
    struct Iter<I: Iterator, K: Eq + Hash, C: FnMut(&I::Item) -> Option<K>> {
        visited: HashSet<K>,
        key: C,
        inside: I,
    }
    impl<I: Iterator, K: Eq + Hash, C: FnMut(&I::Item) -> Option<K>> Iterator for Iter<I, K, C> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let item = self.inside.next()?;
                let key = (self.key)(&item);
                if key.is_some_and(|k| !self.visited.insert(k)) {
                    continue;
                }
                return Some(item);
            }
        }
    }
    Iter {
        visited: HashSet::new(),
        key,
        inside: iter,
    }
}
