pub use crate::{
    node::*,
    error::ParseError::*,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
