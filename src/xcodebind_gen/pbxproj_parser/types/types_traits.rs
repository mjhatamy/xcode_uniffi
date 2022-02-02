use super::end_reason::*;

pub trait PlistItemCustomDeserializeCompatible {
    fn from_plist_result(input: &(&str, Option<String>, PlistIResultEndReason)) -> Self;
}

pub trait XcodePlistSerializer {
    fn serialize(&self) -> String;
}

pub trait InternalXcodePlistSerializer {
    fn serialize(&self, number_of_tabs: &mut i32) -> String;
}
