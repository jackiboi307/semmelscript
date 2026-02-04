pub use crate::{
    node::*,
    parser::Keyword,
    error::ParseError::*,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
