use nom::IResult;

use super::super::types::*;

pub trait XcodeParserDeserializer {
    fn get_plist_item<T: PlistItemCustomDeserializeCompatible>(&self) -> IResult<&str, Box<T>>;
}

impl XcodeParserDeserializer for (&str, (&str, Option<String>, PlistIResultEndReason)) {
    fn get_plist_item<T: PlistItemCustomDeserializeCompatible>(&self) -> IResult<&str, Box<T>> {
        let res: Box<T> = match self.1 .2 {
            PlistIResultEndReason::KeyEnd => Box::new(T::from_plist_result(&self.1)),
            _ => panic!("Unhandled type"),
        };
        Ok((self.0, res))
    }
}

pub trait StringSerializer {
    fn as_comment(&self) -> String;
    fn get_tabs(number_of_tabs: i32) -> String;
}

impl StringSerializer for String {
    fn as_comment(&self) -> String {
        format!("/* {} */", self)
    }

    fn get_tabs(number_of_tabs: i32) -> String {
        (0..number_of_tabs).map(|_| '\t').collect::<String>()
    }
}

impl StringSerializer for Option<String> {
    fn as_comment(&self) -> String {
        if let Some(val) = self {
            val.as_comment()
        } else {
            "".to_string()
        }
    }

    fn get_tabs(number_of_tabs: i32) -> String {
        String::get_tabs(number_of_tabs)
    }
}
