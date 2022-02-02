///
/// End Reasons:
/// ( Tuple Begin
/// ); Tuple End
/// { Array Begin
/// }; Array End
/// ; Value End
/// = Value Begin
/// , Tuple Value End
/// \r\t\n LineFeed ???
/// ' ' Whitespace ???
#[allow(dead_code)]
#[derive(Debug, std::cmp::Eq, std::cmp::PartialEq)]
pub enum PlistIResultEndReason {
    ValueEnd,
    KeyEnd,
    ArrayBegin,
    ArrayEnd,
    TupleBegin,
    TupleEnd,
    TupleValueEnd,
    WhiteSpace,
    LineFeed,
    SectionBegin,
    SectionEnd,
    Eof,
}

impl PlistIResultEndReason {
    #[allow(dead_code)]
    pub fn new(c: char) -> Self {
        match c {
            ' ' => PlistIResultEndReason::WhiteSpace,
            '\t' | '\r' | '\n' => PlistIResultEndReason::LineFeed,
            '=' => PlistIResultEndReason::KeyEnd,
            ',' => PlistIResultEndReason::TupleValueEnd,
            ';' => PlistIResultEndReason::ValueEnd,
            '{' => PlistIResultEndReason::ArrayBegin,
            '}' => PlistIResultEndReason::Eof, // END OF FILE if no trailing semicolon (;)
            '(' => PlistIResultEndReason::TupleBegin,
            ')' => PlistIResultEndReason::TupleEnd,
            _ => panic!("Unknown character: {}", c),
        }
    }
}
