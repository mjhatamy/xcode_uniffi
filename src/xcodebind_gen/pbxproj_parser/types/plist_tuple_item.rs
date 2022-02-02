use crate::xcodebind_gen::helpers::*;
use super::*;

#[derive(Debug, std::cmp::Eq, std::cmp::PartialEq)]
pub struct PlistTupleItem {
    pub name: String,
    pub items: Vec<PlistItem>,
}

impl InternalXcodePlistSerializer for PlistTupleItem {
    fn serialize(&self, number_of_tabs: &mut i32) -> String {
        let line_end_char = if self.items.len() > 1 { "\n" } else { "" };
        let tabs_chars = if self.items.len() > 1 {
            String::get_tabs(*number_of_tabs)
        } else {
            "".to_string()
        };
        let mut string_val = String::new();
        string_val.push_str(
            format!(
                "{}{} = ({}",
                String::get_tabs(*number_of_tabs),
                self.name,
                line_end_char
            )
            .as_str(),
        );
        *number_of_tabs += 1;
        for item in &self.items {
            string_val.push_str(
                format!(
                    " {}{}{}",
                    if self.items.len() > 1 {
                        String::get_tabs(*number_of_tabs)
                    } else {
                        "".to_string()
                    },
                    item.serialize(number_of_tabs),
                    line_end_char
                )
                .as_str(),
            );
        }
        *number_of_tabs -= 1;
        string_val.push_str(format!("{} );\n", tabs_chars).as_str());
        string_val
    }
}

impl PlistTupleItem {
    pub(crate) fn create(
        name: &str,
        items_names_comments: Vec<(&str, Option<String>)>,
    ) -> PlistItem {
        let mut items: Vec<PlistItem> = vec![];
        for (key, val) in items_names_comments {
            items.push(PlistTupleValue::create(key, val))
        }
        PlistItem::TupleItem(PlistTupleItem {
            name: name.to_string(),
            items,
        })
    }
}
