use crate::node::Operator::{self, *};

type Str = &'static str;

// static is used for 'collections' in a broader sense,
// while const is used for individual tokens

pub static DIGITS: Str = "0123456789";
pub static LOWERCASE_LETTERS: Str = "abcdefghijklmnopqrstuvwxyz";
pub static LETTERS: Str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub static IDENTIFIER_CHARS: Str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
pub static STRING_TERMINATORS: Str = "\"'";

// keywords
// only lowercase should be used
pub const KW_LET: Str = "let";
pub static KEYWORDS: &[&str] = &[KW_LET];

// NOTE useful for naming operators
// https://doc.rust-lang.org/book/appendix-02-operators.html

// mathematical operations
pub const OP_ADD: Str = "+";
pub const OP_SUB: Str = "-";
pub const OP_MUL: Str = "*";
pub const OP_DIV: Str = "/";
pub const OP_POW: Str = "^";

// order of operations
pub static OPERATOR_ORDER: &[&[Operator]] = &[
    &[Pow],
    &[Mul, Div], // Mod,
    &[Add, Sub],
    // Equal, Inequal, Less, LessEqual, Greater, GreaterEqual,
    // And,
    // Or,
];

// these come after values
pub const OP_FIELD_ACCESS: Str = ".";
pub const OP_PAREN: Str = "(";
pub static VALUE_EXTENSION_OPERATOR_CHARS: Str = ".(";

// should not be used for checking the type of the following token
pub static OPERATOR_CHARS: Str = "+-*/^().";
