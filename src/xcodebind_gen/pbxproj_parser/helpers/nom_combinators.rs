use nom::{bytes::complete::*, character::complete::*, sequence::*, IResult};

pub trait XcodeParserStringHelper {
    fn validate_header<'a>(&'a self) -> IResult<&'a str, &str>;
    fn get_section_name(&self, is_end: bool) -> IResult<&str, &str>;
}

impl XcodeParserStringHelper for str {
    fn validate_header(&self) -> IResult<&str, &str> {
        let p = tuple((tag("// !$*UTF8*$!"), multispace0, char('{'), multispace0))(self)?;
        Ok((p.0, p.1 .0))
    }

    fn get_section_name(&self, is_end: bool) -> IResult<&str, &str> {
        let end_or_begin = if is_end {
            tag("/* End")
        } else {
            tag("/* Begin")
        };
        let try_type: IResult<&str, (&str, &str, &str)> = tuple((
            multispace0,
            delimited(end_or_begin, take_until("*/"), tag("*/")),
            multispace0,
        ))(self);

        match try_type {
            Ok(res) => {
                // array Type
                Ok((res.0, res.1 .1.trim()))
            }
            Err(e) => {
                Err(e)
                //panic!("Failed")
            }
        }
    }
}
