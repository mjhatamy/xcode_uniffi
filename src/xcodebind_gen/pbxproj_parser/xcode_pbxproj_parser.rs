use nom::{character::complete::multispace0, IResult};

use crate::xcodebind_gen::helpers::*;
use crate::xcodebind_gen::types::*;

#[allow(dead_code)]
struct StringItemSlice {
    begin: usize,
    end: usize,
    begin_set: bool,
    end_set: bool,
}

impl StringItemSlice {
    fn new() -> Self {
        StringItemSlice {
            begin: 0,
            end: 0,
            begin_set: false,
            end_set: false,
        }
    }
    fn is_set(&self) -> bool {
        self.begin_set && self.end_set
    }
    fn is_incomplete(&self) -> bool {
        (self.begin_set != self.end_set) & (self.begin_set | self.end_set)
    }
    fn set_begin(&mut self, val: usize) {
        self.begin = val;
        self.begin_set = true;
    }
    fn set_end(&mut self, val: usize) {
        self.end = val;
        self.end_set = true;
    }
}

pub(crate) struct XcodePbxProject {
    items: Vec<PlistItem>,
}

impl XcodePbxProject {
    pub(crate) fn parse_from_string(content: &str) -> Self {
        let res = content
            .validate_header()
            .expect("Xcode Project plist format is invalid. Header not found");

        let result = XcodePbxProject::parse(res.0).expect("---");
        println!("Parser:\n{}", result.1.serialize());
        XcodePbxProject { items: result.1 }
    }

    fn parse(input: &str) -> IResult<&str, Vec<PlistItem>> {
        let mut input_ref = input;
        let mut max_iter = 0;
        let mut plist_keys_at_array_depth: Vec<Box<PlistKey>> = vec![];
        let mut plist_values_at_array_depth: Vec<Vec<PlistItem>> = vec![vec![]];
        let mut array_depth: usize = 0;
        plist_values_at_array_depth[array_depth] = vec![];
        loop {
            let item = XcodePbxProject::get_item_from_plist(input_ref)?;
            input_ref = item.0;
            match &item.1 .2 {
                PlistIResultEndReason::KeyEnd => {
                    let res = PlistKey::from_plist_result(&item.1);
                    plist_keys_at_array_depth.push(Box::new(res));
                }
                PlistIResultEndReason::ValueEnd => {
                    let res = PlistValue::from_plist_result(&item.1);
                    if let Some(item_key) = plist_keys_at_array_depth.pop() {
                        let key_val_item = PlistItem::Item(PlistKeyValueItem {
                            key: item_key,
                            value: Box::new(PlistItem::Value(res)),
                        });
                        plist_values_at_array_depth[array_depth].push(key_val_item);
                    }
                }
                PlistIResultEndReason::ArrayBegin => {
                    array_depth += 1;
                    plist_values_at_array_depth.push(vec![]);
                }
                PlistIResultEndReason::ArrayEnd => {
                    let values = plist_values_at_array_depth[array_depth].remove_all();
                    array_depth -= 1;
                    let item = PlistItem::Item(PlistKeyValueItem {
                        key: plist_keys_at_array_depth
                            .pop()
                            .expect("Xcode project Plist file is invalid"),
                        value: Box::new(PlistItem::Array(values)),
                    });
                    plist_values_at_array_depth[array_depth].push(item);
                }
                PlistIResultEndReason::TupleBegin => {
                    array_depth += 1;
                    plist_values_at_array_depth.push(vec![]);
                }
                PlistIResultEndReason::TupleValueEnd => {
                    let res = PlistTupleValue::from_plist_result(&item.1);
                    plist_values_at_array_depth[array_depth].push(PlistItem::TupleValue(res));
                }
                PlistIResultEndReason::TupleEnd => {
                    let all_tuples = plist_values_at_array_depth[array_depth].remove_all();
                    array_depth -= 1;
                    let item = PlistItem::TupleItem(PlistTupleItem {
                        name: plist_keys_at_array_depth
                            .pop()
                            .unwrap_or_else(|| panic!("Xcode project Plist file is invalid: \n{:?}", item) )
                            .name,
                        items: all_tuples,
                    });
                    plist_values_at_array_depth[array_depth].push(item);
                }
                // Sections are treated like an array but without initial key fetch.
                PlistIResultEndReason::SectionBegin => {
                    let res = PlistKey::from_plist_result(&item.1);
                    plist_keys_at_array_depth.push(Box::new(res));
                    array_depth += 1;
                    plist_values_at_array_depth.push(vec![]);
                }
                PlistIResultEndReason::SectionEnd => {
                    let values = plist_values_at_array_depth[array_depth].remove_all();
                    array_depth -= 1;
                    let item = PlistItem::SectionItem(PlistSectionItem {
                        name: plist_keys_at_array_depth
                            .pop()
                            .expect("Xcode project Plist file is invalid")
                            .name,
                        items: values,
                    });
                    plist_values_at_array_depth[array_depth].push(item);
                }
                PlistIResultEndReason::Eof => {
                    break;
                }
                _ => panic!("Unhandled type"),
            }

            max_iter += 1;
            if input_ref.len() <= 1 && max_iter < 5000 {
                break;
            }
        }
        Ok(("", plist_values_at_array_depth[0].remove_all()))
    }

    fn get_item_from_plist(
        input: &str,
    ) -> IResult<&str, (&str, Option<String>, PlistIResultEndReason)> {
        let non_ws = multispace0(input)?;
        let input_ref = non_ws.0;

        if let Ok(section_res) = input_ref.get_section_name(false) {
            return Ok((
                section_res.0,
                (section_res.1, None, PlistIResultEndReason::SectionBegin),
            ));
        } else if let Ok(section_res) = input_ref.get_section_name(true) {
            return Ok((
                section_res.0,
                (section_res.1, None, PlistIResultEndReason::SectionEnd),
            ));
        }

        let mut is_inside_quote = false;
        let mut is_parsing_comment = 0;
        let mut comment_begin_end: StringItemSlice = StringItemSlice::new();
        let mut value_begin_end: StringItemSlice = StringItemSlice::new();
        for (i, c) in input_ref.chars().enumerate() {
            if !is_inside_quote
                && is_parsing_comment < 2
                && (c == '\t'
                    || c == '\r'
                    || c == '\n'
                    || c == ';'
                    || c == ','
                    || c == '{'
                    || c == '}'
                    || c == '('
                    || c == ')'
                    || c == '=')
            {
                let mut current_index = i;
                // Form Comment
                let comment_str = if comment_begin_end.is_set() || comment_begin_end.is_incomplete()
                {
                    Some(
                        input_ref[comment_begin_end.begin..comment_begin_end.end]
                            .trim()
                            .to_string(),
                    )
                } else {
                    None
                };

                let value_str = if value_begin_end.is_set() {
                    &input_ref[value_begin_end.begin..value_begin_end.end]
                } else if value_begin_end.is_incomplete() {
                    &input_ref[value_begin_end.begin..current_index]
                } else {
                    ""
                };

                let mut end_reason = PlistIResultEndReason::new(c);
                if c == '}' {
                    if input_ref.chars().nth(current_index + 1) == Some(';') {
                        current_index += 1;
                        end_reason = PlistIResultEndReason::ArrayEnd;
                    }
                } else if c == ')' && input_ref.chars().nth(current_index + 1) == Some(';') {
                    current_index += 1;
                    end_reason = PlistIResultEndReason::TupleEnd;
                }
                return Ok((
                    &input_ref[current_index + 1..],
                    (value_str, comment_str, end_reason),
                ));
            }

            if is_inside_quote && is_parsing_comment < 2 {
                if is_inside_quote && c == '"' && input_ref.chars().nth(i - 1) != Some('\\') {
                    is_inside_quote = false;
                }
                continue;
            }

            if !is_inside_quote && is_parsing_comment == 2 {
                if !comment_begin_end.begin_set && c != ' ' {
                    comment_begin_end.set_begin(i);
                }

                if c == '*' && input_ref.chars().nth(i + 1) == Some('/') {
                    is_parsing_comment = 0;
                    comment_begin_end.set_end(i - 1);
                }
                continue;
            }

            // Ignore whitespaces
            if !value_begin_end.begin_set && c == ' ' {
                continue;
            } else if value_begin_end.is_incomplete() && (c == ' ') {
                value_begin_end.set_end(i);
            }

            // Detect start of a comment (if the comment has no white-space after the value)
            if is_parsing_comment == 0 && c == '/' {
                is_parsing_comment = 1;
            } else if is_parsing_comment == 1 && c == '*' {
                // If comment and value have no spaces between
                if !value_begin_end.end_set {
                    value_begin_end.set_end(i - 1);
                }
                is_parsing_comment = 2;
            } else if is_parsing_comment == 1 && c != '*' {
                is_parsing_comment = 0;
            }

            if !value_begin_end.begin_set {
                value_begin_end.set_begin(i);
            }

            // Quote detection
            if !is_inside_quote && c == '"' {
                if i > 0 && input_ref.chars().nth(i - 1) == Some('\\') {
                    is_inside_quote = false;
                } else {
                    is_inside_quote = true;
                }
            }
        }
        Ok((input_ref, ("", None, PlistIResultEndReason::Eof)))
    }

    pub(crate) fn create(_project_name: String) {}
}
