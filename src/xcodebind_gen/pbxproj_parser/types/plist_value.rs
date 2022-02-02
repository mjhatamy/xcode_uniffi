use crate::xcodebind_gen::helpers::*;

use super::*;

#[derive(Debug, std::cmp::Eq, std::cmp::PartialEq)]
pub struct PlistValue {
    pub value: String,
    pub comment: Option<String>,
}

impl PlistItemCustomDeserializeCompatible for PlistValue {
    fn from_plist_result(input: &(&str, Option<String>, PlistIResultEndReason)) -> Self {
        PlistValue {
            value: input.0.to_string(),
            comment: input.1.clone(),
        }
    }
}

impl InternalXcodePlistSerializer for PlistValue {
    fn serialize(&self, _number_of_tabs: &mut i32) -> String {
        let comment = self.comment.as_comment();
        let space = if comment.is_empty() { "" } else { " " };
        format!("{}{}{};\n", self.value, space, comment)
    }
}
