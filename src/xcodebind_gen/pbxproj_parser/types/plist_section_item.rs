use super::*;

#[derive(Debug, std::cmp::Eq, std::cmp::PartialEq)]
pub struct PlistSectionItem {
    pub name: String,
    pub items: Vec<PlistItem>,
}

impl InternalXcodePlistSerializer for PlistSectionItem {
    fn serialize(&self, number_of_tabs: &mut i32) -> String {
        let mut string_val = String::new();
        string_val.push_str(format!("\n/* Begin {} section */\n", self.name).as_str());
        //*number_of_tabs += 1;
        for item in &self.items {
            string_val.push_str(item.serialize(number_of_tabs).as_str());
        }
        //*number_of_tabs -= 1;
        string_val.push_str(format!("/* End {} section */\n", self.name).as_str());
        string_val
    }
}
