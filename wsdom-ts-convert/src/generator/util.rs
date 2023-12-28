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
        "as" => "as_",
        "break" => "break_",
        "const" => "const_",
        "continue" => "continue_",
        "crate" => "crate_",
        "else" => "else_",
        "enum" => "enum_",
        "extern" => "extern_",
        "false" => "false_",
        "fn" => "fn_",
        "for" => "for_",
        "if" => "if_",
        "impl" => "impl_",
        "in" => "in_",
        "let" => "let_",
        "loop" => "loop_",
        "match" => "match_",
        "mod" => "mod_",
        "move" => "move_",
        "mut" => "mut_",
        "pub" => "pub_",
        "ref" => "ref_",
        "return" => "return_",
        "self" => "self_",
        "Self" => "Self_",
        "static" => "static_",
        "struct" => "struct_",
        "super" => "super_",
        "trait" => "trait_",
        "true" => "true_",
        "type" => "type_",
        "unsafe" => "unsafe_",
        "use" => "use_",
        "where" => "where_",
        "while" => "while_",
        "async" => "async_",
        "await" => "await_",
        "dyn" => "dyn_",
        "abstract" => "abstract_",
        "become" => "become_",
        "box" => "box_",
        "do" => "do_",
        "final" => "final_",
        "macro" => "macro_",
        "override" => "override_",
        "priv" => "priv_",
        "typeof" => "typeof_",
        "unsized" => "unsized_",
        "virtual" => "virtual_",
        "yield" => "yield_",
        "try" => "try_",
        n => n,
    };
    let name = name.replace('-', "_");
    Ident::new(&name, Span::call_site())
}

pub fn iter_dedupe_consecutive<I: Iterator, K: Eq>(
    iter: I,
    key: impl FnMut(&I::Item) -> K,
) -> impl Iterator<Item = I::Item> {
    struct Iter<I: Iterator, C> {
        key: C,
        inside: std::iter::Peekable<I>,
    }
    impl<I: Iterator, K: Eq, C: FnMut(&I::Item) -> K> Iterator for Iter<I, C> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let item = self.inside.next()?;
                let item_key = (self.key)(&item);
                let next_dup = self
                    .inside
                    .peek()
                    .is_some_and(|next| (self.key)(next) == item_key);
                if !next_dup {
                    return Some(item);
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
    key: impl FnMut(&I::Item) -> K,
) -> impl Iterator<Item = I::Item> {
    struct Iter<I: Iterator, K: Eq + Hash, C: FnMut(&I::Item) -> K> {
        visited: HashSet<K>,
        key: C,
        inside: I,
    }
    impl<I: Iterator, K: Eq + Hash, C: FnMut(&I::Item) -> K> Iterator for Iter<I, K, C> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let item = self.inside.next()?;
                let key = (self.key)(&item);
                if !self.visited.insert(key) {
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
