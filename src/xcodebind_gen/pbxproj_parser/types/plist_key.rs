use std::fmt;
use std::fmt::{Debug, Formatter};

use super::*;
use crate::xcodebind_gen::helpers::*;

#[derive(Debug, std::cmp::Eq, std::cmp::PartialEq)]
pub struct PlistKey {
    pub name: String,
    pub comment: Option<String>,
}

impl PlistItemCustomDeserializeCompatible for PlistKey {
    fn from_plist_result(input: &(&str, Option<String>, PlistIResultEndReason)) -> Self {
        PlistKey {
            name: input.0.to_string(),
            comment: input.1.clone(),
        }
    }
}

impl fmt::Display for PlistKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.comment.as_comment())
    }
}

impl InternalXcodePlistSerializer for PlistKey {
    fn serialize(&self, _number_of_tabs: &mut i32) -> String {
        let comment = self.comment.as_comment();
        let space = if comment.len() > 0 { " " } else { "" };
        format!("{}{}{}{}", self.name, space, comment, space)
    }
}
