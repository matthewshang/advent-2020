use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Field {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

impl Field {
    fn is_valid(&self, value: &str) -> bool {
        lazy_static! {
            static ref BYR: Regex = Regex::new("^(19[2-9][0-9])|(200[0-2])$").unwrap();
            static ref IYR: Regex = Regex::new("^20(1[0-9]|20)$").unwrap();
            static ref EYR: Regex = Regex::new("^20(2[0-9]|30)$").unwrap();
            static ref HGT: Regex = Regex::new("^1([5-8][0-9]|(9[0-3]))cm|(59|6[0-9]|7[0-6])in$").unwrap();
            static ref HCL: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
            static ref ECL: Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref PID: Regex = Regex::new("^[0-9]{9}$").unwrap();
        }

        let res = match self {
            Field::BirthYear => BYR.is_match(value),
            Field::IssueYear => IYR.is_match(value),
            Field::ExpirationYear => EYR.is_match(value),
            Field::Height => HGT.is_match(value),
            Field::HairColor => HCL.is_match(value),
            Field::EyeColor => ECL.is_match(value),
            Field::PassportID => PID.is_match(value),
            Field::CountryID => true,
        };
        return res;
    }
}

impl FromStr for Field {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "byr" => Ok(Field::BirthYear),
            "iyr" => Ok(Field::IssueYear),
            "eyr" => Ok(Field::ExpirationYear),
            "hgt" => Ok(Field::Height),
            "hcl" => Ok(Field::HairColor),
            "ecl" => Ok(Field::EyeColor),
            "pid" => Ok(Field::PassportID),
            "cid" => Ok(Field::CountryID),
            _ => Err(format!("'{}' is not a valid value for Field", s)),
        }
    }
}

#[derive(Debug)]
pub struct Passport {
    fields: HashMap<Field, String>,
}

impl Passport {
    fn valid_count(&self) -> bool {
        match self.fields.len() {
            8 => true,
            7 => !self.fields.contains_key(&Field::CountryID),
            _ => false,
        }
    }

    fn valid_fields(&self) -> bool {
        self.fields
            .iter()
            .all(|(k, v)| k.is_valid(&v))
    }
}

fn parse(input: &str) -> Option<Passport> {
    lazy_static! {
        static ref RE: Regex = Regex::new("([a-z]{3}):([#\\w]+)").unwrap();
    }

    let mut fields: HashMap<Field, String> = HashMap::new();
    for cap in RE.captures_iter(input) {
        if let Ok(f) = cap[1].parse() {
            fields.insert(f, cap[2].to_owned());
        }
    }

    Some(Passport {
        fields
    })
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(str::to_owned)
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    input
        .into_iter()
        .map(|l| parse(l).unwrap())
        .filter(|p| p.valid_count())
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<String>) -> usize {
    input
        .into_iter()
        .map(|l| parse(l).unwrap())
        .filter(|p| p.valid_count() && p.valid_fields())
        .count()
}
