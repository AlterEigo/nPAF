use crate::gedcom::{GedLine,Record,ParseError};
use std::io::{BufReader, BufRead};

extern crate regex;
use regex::Regex;

type Predicate = dyn Fn(&GedLine) -> bool;

enum State {
    Initial,
    Reference {records: Vec<Record>},
    RecordTag {records: Vec<Record>, level: i32},
    Invalid
}

impl State {
    fn handle_initial(line: &GedLine) -> Self {
        let cond: &Predicate = &|line| {
            let s = String::from("HEAD");
            match line {
                GedLine::Data(0, s, None) => true,
                _ => false
            }
        };
        if cond(line) {
            Self::Reference {records: Default::default()}
        } else {
            Self::Invalid
        }
    }

    fn handle_ref(recs: Vec<Record>, line: &GedLine) -> Self {
        let cond: &Predicate = &|line| {
            if let GedLine::Ref(lvl, _, _, _) = line {
                if *lvl == 0 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if cond(line) {
            Self::RecordTag {records: recs, level: 1}
        } else {
            Self::Invalid
        }
    }

    fn handle_tag(recs: Vec<Record>, lvl: i32, line: &GedLine) -> Self {
        match line {
            GedLine::Ref(nlvl, _, _, _) => {
                if *nlvl == 0 {
                    Self::RecordTag {records: recs, level: 1}
                } else {
                    Self::Invalid
                }
            },
            GedLine::Data(nlvl, _, _) => {
                if *nlvl == (lvl + 1) || *nlvl == lvl {
                    Self::RecordTag {records: recs, level: *nlvl}
                } else {
                    Self::Invalid
                }
            }
        }
    }

    fn handle_invalid(line: &GedLine) -> Self {
        Self::Invalid
    }

    pub fn next(self, line: &GedLine) -> Self {
        match self {
            Self::Initial => Self::handle_initial(line),
            Self::Reference {records} => Self::handle_ref(records, line),
            Self::RecordTag {records, level} => Self::handle_tag(records, level, line),
            Self::Invalid => Self::handle_invalid(line)
        }
    }

    pub fn can_advance(&self) -> bool {
        match self {
            Self::Initial | Self::Reference {..} | Self::RecordTag {..} => true,
            Self::Invalid => false
        }
    }

    pub fn successful(&self) -> bool {
        match self {
            Self::Initial | Self::Reference {..} | Self::Invalid => false,
            Self::RecordTag {..} => true
        }
    }

    pub fn fold(self) -> Result<Vec<Record>, ParseError> {
        match self {
            Self::Initial | Self::Invalid => Err(ParseError::Runtime(String::from("Not GEDCOM data."))),
            Self::Reference {records} | Self::RecordTag {records, ..} => Ok(records)
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Initial
    }
}

#[derive(Default)]
pub struct GedEx {
    contents: Vec<String>
}

impl GedEx {
    fn new(contents: Vec<String>) -> Self {
        GedEx {
            contents: contents,
            ..Default::default()
        }
    }

    fn parse(self) -> Result<Vec<Record>, ParseError> {
        let records = self.contents.into_iter()
            .filter_map(|line| Self::parse_line(&line))
            .fold(State::Initial, |state, line| {
                state.next(&line)
            })
            .fold()?;
        Ok(records)
    }

    /// Method allowing to count all the lines that can't
    /// be parsed by the `parse` method.
    pub fn count_unparsed(file: &std::fs::File) -> i64 {
        let reader = BufReader::new(file);
        let re = Self::regex_line();
        let re_ref = Self::regex_ref();
        reader.lines()
            .filter_map(|l| l.ok())
            .filter(|l| !re.is_match(&l) && !re_ref.is_match(&l))
            .inspect(|l| println!("Unmatched: '{}'", l))
            .fold(0, |acc, _l| acc + 1)
    }

    /// Private subroutine that takes a raw string and
    /// parses it into an interpreted line with data
    fn parse_line(line: &str) -> Option<GedLine> {
        let r_data = Self::regex_line();
        let r_ref = Self::regex_ref();

        if let Some(caps) = r_data.captures(&line) {
            Some(GedLine::Data(
                caps.name("Level").unwrap().as_str().parse().unwrap(),
                caps.name("Tag").unwrap().as_str().to_owned(),
                caps.name("Content").map(|s| s.as_str().to_owned())
            ))
        } else if let Some(caps) = r_ref.captures(&line) {
            Some(GedLine::Ref(
                caps.name("Level").unwrap().as_str().parse().unwrap(),
                caps.name("Type").unwrap().as_str().to_owned(),
                caps.name("Number").unwrap().as_str().parse().unwrap(),
                caps.name("Content").map(|s| s.as_str().to_owned())
                // if let Some(content) = caps.name("Content") { Some(content.as_str().to_owned()) } else { None }
            ))
        } else {
            None
        }
    }

    /// Line regular expression getter
    fn regex_line() -> Regex {
        Regex::new(r"(?x) # Insignificant whitespace mode
                ^
                (?P<Level>[0-9]{1,2})\ *       # Line level
                (?P<Tag>_?[A-Z]{3,5})\ *       # Record tag
                (?P<Content>[^\r\n]+)*         # Either end of line or content
                $
            ").unwrap()
    }

    /// Reference regular expression getter
    fn regex_ref() -> Regex {
        Regex::new(r"(?x)
                ^
                (?P<Level>[0-9]{1,2})\ *               # Line level
                @(?P<Type>[A-Z]+)(?P<Number>\d+)@\ *   # Record tag
                (?P<Content>[^\r\n]+)?                 # Either end of line or content
                $
            ").unwrap()
    }
}
