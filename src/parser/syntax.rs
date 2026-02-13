use super::tokens::Operator::{self, *};

type Str = &'static str;

// static is used for 'collections' in a broader sense,
// while const is used for individual tokens

pub static DIGITS: Str = "0123456789";
pub static LOWERCASE_LETTERS: Str = "abcdefghijklmnopqrstuvwxyz";
pub static LETTERS: Str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub static IDENTIFIER_CHARS: Str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

pub static STRING_TERMINATORS: Str = "\"'";
pub static EXPR_TERMINATORS: Str = "),}";

// keywords
// only lowercase should be used
pub const KW_LET: Str = "let";
pub const KW_IF: Str = "if";
pub const KW_ELIF: Str = "elif";
pub const KW_ELSE: Str = "else";
pub const KW_FUNC: Str = "fn";
pub const KW_TRUE: Str = "true";
pub const KW_FALSE: Str = "false";
pub static KEYWORDS: &[&str] = &[
    KW_LET, KW_IF, KW_ELIF, KW_ELSE, KW_FUNC, KW_TRUE, KW_FALSE
];

// NOTE useful for naming operators
// https://doc.rust-lang.org/book/appendix-02-operators.html

// NOTE some operators like () and . lack constants

pub const OP_ADD: Str = "+";
pub const OP_SUB: Str = "-";
pub const OP_MUL: Str = "*";
pub const OP_DIV: Str = "/";
pub const OP_POW: Str = "^";
pub const OP_MOD: Str = "%";

pub const OP_EQUAL: Str = "==";
pub const OP_INEQUAL: Str = "!=";
pub const OP_LESS: Str = "<";
pub const OP_LESSEQUAL: Str = "<=";
pub const OP_GREATER: Str = ">";
pub const OP_GREATEREQUAL: Str = ">=";

pub const OP_AND: Str = "&&";
pub const OP_OR: Str = "||";

pub const OP_SETVALUE: Str = "=";

// order of operations
pub static OPERATOR_ORDER: &[&[Operator]] = &[
    &[SetValue],
    &[Pow],
    &[Mul, Div, Mod],
    &[Add, Sub],
    &[Equal, Inequal, Less, LessEqual, Greater, GreaterEqual],
    &[And],
    &[Or],
];

// should not be used for checking the type of the following token
pub static OPERATOR_CHARS: Str = "+-*/^%().!=<>&|";
