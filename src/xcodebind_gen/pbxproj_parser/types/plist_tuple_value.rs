use crate::xcodebind_gen::helpers::*;

use super::*;

#[derive(Debug, std::cmp::Eq, std::cmp::PartialEq)]
pub struct PlistTupleValue {
    pub value: String,
    pub comment: Option<String>,
}

impl PlistItemCustomDeserializeCompatible for PlistTupleValue {
    fn from_plist_result(input: &(&str, Option<String>, PlistIResultEndReason)) -> Self {
        PlistTupleValue {
            value: input.0.to_string(),
            comment: input.1.clone(),
        }
    }
}

impl InternalXcodePlistSerializer for PlistTupleValue {
    fn serialize(&self, _number_of_tabs: &mut i32) -> String {
        let comment = self.comment.as_comment();
        let space = if comment.is_empty() { "" } else { " " };
        format!("{}{}{}{},", self.value, space, comment, space)
    }
}

impl PlistTupleValue {
    pub(crate) fn create(value: &str, comment: Option<String>) -> PlistItem {
        PlistItem::TupleValue(PlistTupleValue {
            value: value.to_string(),
            comment,
        })
    }
}
