//! FIXME: write short doc here

use std::fmt;

use ra_syntax::{ast, SmolStr};

/// `Name` is a wrapper around string, which is used in hir for both references
/// and declarations. In theory, names should also carry hygiene info, but we are
/// not there yet!
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(Repr);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Repr {
    Text(SmolStr),
    TupleField(usize),
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Repr::Text(text) => fmt::Display::fmt(&text, f),
            Repr::TupleField(idx) => fmt::Display::fmt(&idx, f),
        }
    }
}

impl Name {
    /// Note: this is private to make creating name from random string hard.
    /// Hopefully, this should allow us to integrate hygiene cleaner in the
    /// future, and to switch to interned representation of names.
    const fn new_text(text: SmolStr) -> Name {
        Name(Repr::Text(text))
    }

    pub(crate) fn new_tuple_field(idx: usize) -> Name {
        Name(Repr::TupleField(idx))
    }

    /// Shortcut to create inline plain text name
    const fn new_inline_ascii(len: usize, text: &[u8]) -> Name {
        Name::new_text(SmolStr::new_inline_from_ascii(len, text))
    }

    /// Resolve a name from the text of token.
    fn resolve(raw_text: &SmolStr) -> Name {
        let raw_start = "r#";
        if raw_text.as_str().starts_with(raw_start) {
            Name::new_text(SmolStr::new(&raw_text[raw_start.len()..]))
        } else {
            Name::new_text(raw_text.clone())
        }
    }

    pub(crate) fn missing() -> Name {
        Name::new_text("[missing name]".into())
    }

    pub(crate) fn as_tuple_index(&self) -> Option<usize> {
        match self.0 {
            Repr::TupleField(idx) => Some(idx),
            _ => None,
        }
    }
}

pub(crate) trait AsName {
    fn as_name(&self) -> Name;
}

impl AsName for ast::NameRef {
    fn as_name(&self) -> Name {
        match self.as_tuple_field() {
            Some(idx) => Name::new_tuple_field(idx),
            None => Name::resolve(self.text()),
        }
    }
}

impl AsName for ast::Name {
    fn as_name(&self) -> Name {
        Name::resolve(self.text())
    }
}

impl AsName for ast::FieldKind {
    fn as_name(&self) -> Name {
        match self {
            ast::FieldKind::Name(nr) => nr.as_name(),
            ast::FieldKind::Index(idx) => Name::new_tuple_field(idx.text().parse().unwrap()),
        }
    }
}

impl AsName for ra_db::Dependency {
    fn as_name(&self) -> Name {
        Name::new_text(self.name.clone())
    }
}

// Primitives
pub(crate) const ISIZE: Name = Name::new_inline_ascii(5, b"isize");
pub(crate) const I8: Name = Name::new_inline_ascii(2, b"i8");
pub(crate) const I16: Name = Name::new_inline_ascii(3, b"i16");
pub(crate) const I32: Name = Name::new_inline_ascii(3, b"i32");
pub(crate) const I64: Name = Name::new_inline_ascii(3, b"i64");
pub(crate) const I128: Name = Name::new_inline_ascii(4, b"i128");
pub(crate) const USIZE: Name = Name::new_inline_ascii(5, b"usize");
pub(crate) const U8: Name = Name::new_inline_ascii(2, b"u8");
pub(crate) const U16: Name = Name::new_inline_ascii(3, b"u16");
pub(crate) const U32: Name = Name::new_inline_ascii(3, b"u32");
pub(crate) const U64: Name = Name::new_inline_ascii(3, b"u64");
pub(crate) const U128: Name = Name::new_inline_ascii(4, b"u128");
pub(crate) const F32: Name = Name::new_inline_ascii(3, b"f32");
pub(crate) const F64: Name = Name::new_inline_ascii(3, b"f64");
pub(crate) const BOOL: Name = Name::new_inline_ascii(4, b"bool");
pub(crate) const CHAR: Name = Name::new_inline_ascii(4, b"char");
pub(crate) const STR: Name = Name::new_inline_ascii(3, b"str");

// Special names
pub(crate) const SELF_PARAM: Name = Name::new_inline_ascii(4, b"self");
pub(crate) const SELF_TYPE: Name = Name::new_inline_ascii(4, b"Self");
pub(crate) const MACRO_RULES: Name = Name::new_inline_ascii(11, b"macro_rules");

// Components of known path (value or mod name)
pub(crate) const STD: Name = Name::new_inline_ascii(3, b"std");
pub(crate) const ITER: Name = Name::new_inline_ascii(4, b"iter");
pub(crate) const OPS: Name = Name::new_inline_ascii(3, b"ops");
pub(crate) const FUTURE: Name = Name::new_inline_ascii(6, b"future");
pub(crate) const RESULT: Name = Name::new_inline_ascii(6, b"result");
pub(crate) const BOXED: Name = Name::new_inline_ascii(5, b"boxed");

// Components of known path (type name)
pub(crate) const INTO_ITERATOR_TYPE: Name = Name::new_inline_ascii(12, b"IntoIterator");
pub(crate) const ITEM_TYPE: Name = Name::new_inline_ascii(4, b"Item");
pub(crate) const TRY_TYPE: Name = Name::new_inline_ascii(3, b"Try");
pub(crate) const OK_TYPE: Name = Name::new_inline_ascii(2, b"Ok");
pub(crate) const FUTURE_TYPE: Name = Name::new_inline_ascii(6, b"Future");
pub(crate) const RESULT_TYPE: Name = Name::new_inline_ascii(6, b"Result");
pub(crate) const OUTPUT_TYPE: Name = Name::new_inline_ascii(6, b"Output");
pub(crate) const TARGET_TYPE: Name = Name::new_inline_ascii(6, b"Target");
pub(crate) const BOX_TYPE: Name = Name::new_inline_ascii(3, b"Box");
