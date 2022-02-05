#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate lazy_static;

pub mod colours;
pub mod tartan_register_shades;
use colours::{Sett, Span};
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(?P<colour>[ROYGBPWNKTDLk]+)(?P<pivot>/)?(?P<count>[0-9]+)").unwrap();
}

fn _parse(input: &str, mut sett: Sett) -> (&str, Sett) {
    if let Some(caps) = RE.captures(input) {
        let span = if caps.name("pivot").is_some() {
            Span::pivot
        } else {
            Span::new
        };
        sett.pattern.push(span(&caps["colour"], &caps["count"]));
        let end = caps.get(0).unwrap().end();

        _parse(input.split_at(end).1.trim(), sett)
    } else {
        (input, sett)
    }
}

pub fn parse(input: &str) -> Sett {
    _parse(input.trim(), Sett::new()).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_colour() {
        assert!(parse("X").pattern.is_empty());
    }

    #[test]
    fn one_colour() {
        let sett = parse("R2");
        assert_eq!(sett.pattern[0], Span::new("R", "2"));
    }

    #[test]
    fn two_colours() {
        let sett = parse("R4 G8");
        assert_eq!(sett.pattern[0], Span::new("R", "4"));
        assert_eq!(sett.pattern[1], Span::new("G", "8"));
    }

    #[test]
    fn two_colours_no_space() {
        let sett = parse("Bk6G12");
        assert_eq!(sett.pattern[0], Span::new("Bk", "6"));
        assert_eq!(sett.pattern[1], Span::new("G", "12"));
    }

    #[test]
    fn with_pivot() {
        let sett = parse("P/2");
        assert_eq!(sett.pattern[0], Span::pivot("P", "2"));
    }
}
