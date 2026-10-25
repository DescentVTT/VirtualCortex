//! Exploration 002 (ADR-0193), on `explore/002-what-the-pinned-tables-cost` alone: the rounds'
//! pinned tables read from a data file at run time, where `main` writes them as `const` items.
//!
//! A data file holds the tables a test file moved out of its source, each as the line `@ NAME`
//! followed by the table's value as the source wrote it, character for character: integers,
//! `true` and `false`, `None` and `Some(..)`, tuples, arrays, `[x; n]` and `&[..]`. A table is a
//! `LazyLock` of its own `const`'s type, parsed from that text the first time a test reads it; a
//! value that does not fit its type, a length that is not the array's, or text left after the
//! value panics, so no table is read as anything but what was written.

use std::collections::HashMap;

/// A data file's tables by name, each the text of its value.
pub(crate) struct DataFile {
    entries: HashMap<&'static str, &'static str>,
}

impl DataFile {
    /// The tables of a data file's text.
    pub(crate) fn from_text(text: &'static str) -> Self {
        let mut entries = HashMap::new();
        for chunk in text.split("\n@ ").skip(1) {
            let (name, value) = chunk.split_once('\n').expect("a table's name and its value");
            assert!(entries.insert(name.trim(), value).is_none(), "the table {name} twice");
        }
        DataFile { entries }
    }

    /// The tables of the data file at `path`, read once and kept for the process.
    pub(crate) fn from_file(path: &str) -> Self {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
        DataFile::from_text(Box::leak(text.into_boxed_str()))
    }

    /// The table `name`, parsed as `T`.
    pub(crate) fn read<T: Lit>(&self, name: &str) -> T {
        let text = self.entries.get(name).unwrap_or_else(|| panic!("no table {name}"));
        let mut p = Parser {
            text: text.as_bytes(),
            at: 0,
        };
        let value = T::lit(&mut p);
        p.skip();
        assert!(p.at == p.text.len(), "the table {name} has text after its value at byte {}", p.at);
        value
    }

    /// How many tables the file holds.
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }
}

/// A cursor over one table's text.
pub(crate) struct Parser<'a> {
    text: &'a [u8],
    at: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.text.get(self.at).copied()
    }

    fn bump(&mut self) {
        self.at = self.at.checked_add(1).expect("a position in the text");
    }

    /// Past whitespace and `//` comments.
    fn skip(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.bump();
            } else if c == b'/' && self.text.get(self.at.wrapping_add(1)) == Some(&b'/') {
                while let Some(c) = self.peek() {
                    if c == b'\n' {
                        break;
                    }
                    self.bump();
                }
            } else {
                break;
            }
        }
    }

    /// Whether the next token is `c`, taken if it is.
    fn eat(&mut self, c: u8) -> bool {
        self.skip();
        if self.peek() == Some(c) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, c: u8) {
        assert!(self.eat(c), "expected `{}` at byte {}", c as char, self.at);
    }

    fn word(&mut self) -> &str {
        self.skip();
        let start = self.at;
        while matches!(self.peek(), Some(c) if c.is_ascii_alphabetic()) {
            self.bump();
        }
        std::str::from_utf8(&self.text[start..self.at]).expect("an ASCII word")
    }

    fn integer(&mut self) -> i128 {
        self.skip();
        let negative = self.eat(b'-');
        let mut value: i128 = 0;
        let mut digits = 0_u32;
        while let Some(c) = self.peek() {
            if c == b'_' {
                self.bump();
                continue;
            }
            if !c.is_ascii_digit() {
                break;
            }
            value = value
                .checked_mul(10)
                .and_then(|v| v.checked_add(i128::from(c.wrapping_sub(b'0'))))
                .expect("an integer within i128");
            digits = digits.saturating_add(1);
            self.bump();
        }
        assert!(digits > 0, "expected an integer at byte {}", self.at);
        if negative { value.checked_neg().expect("an integer within i128") } else { value }
    }

    /// The items of `[a, b, c]` or `[x; n]` once `[` is taken, through the closing `]`.
    fn items<T: Lit + Clone>(&mut self) -> Vec<T> {
        let mut out = Vec::new();
        if self.eat(b']') {
            return out;
        }
        let first = T::lit(self);
        if self.eat(b';') {
            let n = usize::try_from(self.integer()).expect("a repetition count");
            self.expect(b']');
            return vec![first; n];
        }
        out.push(first);
        loop {
            if self.eat(b']') {
                return out;
            }
            self.expect(b',');
            if self.eat(b']') {
                return out;
            }
            out.push(T::lit(self));
        }
    }
}

/// A value a table's text can hold.
pub(crate) trait Lit: Sized {
    fn lit(p: &mut Parser<'_>) -> Self;
}

macro_rules! lit_integer {
    ($($t:ty),*) => {$(
        impl Lit for $t {
            fn lit(p: &mut Parser<'_>) -> Self {
                let at = p.at;
                <$t>::try_from(p.integer())
                    .unwrap_or_else(|_| panic!("an integer out of {}'s range at byte {at}", stringify!($t)))
            }
        }
    )*};
}
lit_integer!(u8, u16, u32, u64, usize, i8, i16, i32, i64);

impl Lit for bool {
    fn lit(p: &mut Parser<'_>) -> Self {
        let at = p.at;
        match p.word() {
            "true" => true,
            "false" => false,
            w => panic!("expected a bool at byte {at}, found `{w}`"),
        }
    }
}

impl<T: Lit> Lit for Option<T> {
    fn lit(p: &mut Parser<'_>) -> Self {
        let at = p.at;
        match p.word() {
            "None" => None,
            "Some" => {
                p.expect(b'(');
                let value = T::lit(p);
                p.eat(b',');
                p.expect(b')');
                Some(value)
            }
            w => panic!("expected an Option at byte {at}, found `{w}`"),
        }
    }
}

impl<T: Lit + Clone, const N: usize> Lit for [T; N] {
    fn lit(p: &mut Parser<'_>) -> Self {
        let at = p.at;
        p.expect(b'[');
        let items: Vec<T> = p.items();
        let n = items.len();
        items
            .try_into()
            .unwrap_or_else(|_| panic!("an array of {n} where {N} are declared, at byte {at}"))
    }
}

impl<T: Lit + Clone + 'static> Lit for &'static [T] {
    fn lit(p: &mut Parser<'_>) -> Self {
        p.expect(b'&');
        p.expect(b'[');
        let items: Vec<T> = p.items();
        Box::leak(items.into_boxed_slice())
    }
}

macro_rules! lit_tuple {
    ($($t:ident),+) => {
        impl<$($t: Lit),+> Lit for ($($t,)+) {
            fn lit(p: &mut Parser<'_>) -> Self {
                p.expect(b'(');
                let value = ($({
                    let v = $t::lit(p);
                    p.eat(b',');
                    v
                },)+);
                p.expect(b')');
                value
            }
        }
    };
}
lit_tuple!(A);
lit_tuple!(A, B);
lit_tuple!(A, B, C);
lit_tuple!(A, B, C, D);
lit_tuple!(A, B, C, D, E);
lit_tuple!(A, B, C, D, E, F);
lit_tuple!(A, B, C, D, E, F, G);
lit_tuple!(A, B, C, D, E, F, G, H);
lit_tuple!(A, B, C, D, E, F, G, H, I);
lit_tuple!(A, B, C, D, E, F, G, H, I, J);
lit_tuple!(A, B, C, D, E, F, G, H, I, J, K);
lit_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
lit_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
lit_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
