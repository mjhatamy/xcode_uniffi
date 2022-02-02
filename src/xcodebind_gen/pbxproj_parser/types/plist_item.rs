use crate::xcodebind_gen::helpers::*;

use super::*;

#[allow(dead_code)]
#[derive(Debug, std::cmp::Eq, std::cmp::PartialEq)]
pub enum PlistItem {
    Value(PlistValue),
    TupleValue(PlistTupleValue),
    Item(PlistKeyValueItem),
    SectionItem(PlistSectionItem),
    TupleItem(PlistTupleItem),
    Array(Vec<PlistItem>),
}

impl InternalXcodePlistSerializer for PlistItem {
    fn serialize(&self, number_of_tabs: &mut i32) -> String {
        let mut string_val = String::new();

        match self {
            PlistItem::Value(item) => string_val.push_str(item.serialize(number_of_tabs).as_str()),
            PlistItem::Item(value) => {
                string_val.push_str(value.serialize(number_of_tabs).as_str());
            }
            PlistItem::SectionItem(value) => {
                string_val.push_str((*value).serialize(number_of_tabs).as_str());
            }
            PlistItem::TupleItem(value) => {
                string_val.push_str(value.serialize(number_of_tabs).as_str());
            }
            PlistItem::TupleValue(value) => {
                string_val.push_str(value.serialize(number_of_tabs).as_str());
            }
            PlistItem::Array(value) => {
                string_val.push('{');
                if !value.is_empty() {
                    string_val.push('\n');
                    *number_of_tabs += 1;
                    for litem in value {
                        string_val.push_str(litem.serialize(number_of_tabs).as_str());
                    }
                    *number_of_tabs -= 1;
                    string_val
                        .push_str(format!("{}}};\n", String::get_tabs(*number_of_tabs)).as_str());
                } else {
                    string_val
                        .push_str(format!("{}}};\n", String::get_tabs(*number_of_tabs)).as_str());
                }
            }
        }

        string_val
    }
}

impl XcodePlistSerializer for Vec<PlistItem> {
    fn serialize(&self) -> String {
        let mut number_of_tabs = 2;
        let mut string_val = String::new();
        string_val.push_str("// !$*UTF8*$!\n{\n");
        for item in self {
            string_val.push_str(item.serialize(&mut number_of_tabs).as_str());
        }
        string_val.push_str("\n}\n");
        string_val
    }
}
