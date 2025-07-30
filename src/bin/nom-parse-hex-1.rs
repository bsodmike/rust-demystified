#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Error;

pub fn main() -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
mod tests_3 {
    use nom::{
        bytes::complete::{tag, take_while_m_n},
        combinator::map_res,
        error::{context, convert_error},
        sequence::Tuple,
        IResult, Parser,
    };

    pub fn parse_hex_seg(input: &str) -> IResult<&str, u8, nom::error::VerboseError<&str>> {
        let parse_two_hex_digits_parser_fn = take_while_m_n(
            // RA block
            2,
            2,
            |it: char| it.is_ascii_hexdigit(),
        );

        let mut map_res_fn = map_res(
            // RA block
            parse_two_hex_digits_parser_fn,
            |it| u8::from_str_radix(it, 16),
        );

        map_res_fn.parse(input)
    }

    /// `nom` is used to parse the hex digits from string. Then [u8::from_str_radix] is
    /// used to convert the hex string into a number. This can't fail, even though in the
    /// function signature, that may return a [core::num::ParseIntError], which never
    /// happens. Note the use of [nom::error::VerboseError] to get more detailed error
    /// messages that are passed to [nom::error::convert_error].
    ///
    /// Even if [core::num::ParseIntError] were to be thrown, it would be consumed, and
    /// a higher level `nom` error would be returned for the `map_res` combinator.
    pub fn parse_hex_seg_combined(
        input: &str,
    ) -> IResult<&str, u8, nom::error::VerboseError<&str>> {
        map_res(
            take_while_m_n(2, 2, |it: char| it.is_ascii_hexdigit()),
            |it| u8::from_str_radix(it, 16),
        )
        .parse(input)
    }

    /// Note the use of [nom::error::VerboseError] to get more detailed error messages
    /// that are passed to [nom::error::convert_error].
    pub fn root(input: &str) -> IResult<&str, (u8, u8, u8), nom::error::VerboseError<&str>> {
        let (remainder, (_, red, green, blue)) = (
            context("start of hex color", tag("#")),
            context("hex seg 1", parse_hex_seg),
            context("hex seg 2", parse_hex_seg),
            context("hex seg 3", parse_hex_seg),
        )
            .parse(input)?;

        Ok((remainder, (red, green, blue)))
    }

    #[test]
    fn test_root_1() {
        let input = "x#FF0000";
        let result = root(input);
        println!("{:?}", result);
        assert!(result.is_err());

        match result {
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                println!("Could not parse because ... {}", convert_error(input, e));
            }
            _ => { /* do nothing for nom::Err::Incomplete(_) */ }
        }
    }

    #[test]
    fn test_root_2() {
        let input = "#FF_0000";
        let result = root(input);
        println!("{:?}", result);
        assert!(result.is_err());

        match result {
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                println!("Could not parse because ... {}", convert_error(input, e));
            }
            _ => { /* do nothing for nom::Err::Incomplete(_) */ }
        }
    }

    #[test]
    fn test_root_3() {
        let input = "#FF00AA";
        let result = root(input);
        println!("{:?}", result);
        assert!(result.is_ok());

        match result {
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                println!("Could not parse because ... {}", convert_error(input, e));
            }
            _ => { /* do nothing for nom::Err::Incomplete(_) */ }
        }
    }
}
