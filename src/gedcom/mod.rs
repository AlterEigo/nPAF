use std::rc::Rc;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cell::RefCell;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

pub struct Record {
    id: u64,
    name: String,
    father: Option<Rc<RefCell<Record>>>,
    mother: Option<Rc<RefCell<Record>>>,
    children: Vec<Rc<RefCell<Record>>>
}

type IOError = std::io::Error;

pub enum ParseError {
    IO(IOError)
}

enum GedLine {
    Data(i32, String, Option<String>),
    Ref(i32, String, Option<String>)
}

impl From<IOError> for ParseError {
    fn from(o: IOError) -> ParseError {
        ParseError::IO(o)
    }
}

pub type RecordRegistry = HashMap<u64, Rc<RefCell<Record>>>;
pub type ParseResult = Result<RecordRegistry, ParseError>;

pub trait Parser {
    type FileType;

    fn parse(&mut self, file: &Self::FileType) -> ParseResult;
}

#[derive(Default)]
pub struct GedParser {

}

impl GedParser {
    fn regex_line() -> Regex {
        Regex::new(r"(?x) # Insignificant whitespace mode
                ^
                (?P<Level>\d{1,2})\s              # Line level
                (?P<Tag>_?[A-Z]{3,5})             # Record tag
                \s*$   |   (?P<Content>[^\r\n]*)  # Either end of line or content
                $
            ").unwrap()
    }

    fn regex_ref() -> Regex {
        Regex::new(r"(?x)
                ^
                (?P<Level>\d{1,2})\s              # Line level
                (?P<Tag>@[A-Z]+\d+@)              # Record tag
                \s*$   |   (?P<Content>[^\r\n]*)  # Either end of line or content
                $
            ").unwrap()
    }
}

impl Buildable for GedParser {
    type BuilderType = GedParserBuilder;
}

impl Parser for GedParser {
    type FileType = std::fs::File;

    fn parse(&mut self, file: &Self::FileType) -> ParseResult {
        let mut reader = BufReader::new(file);
        let re = GedParser::regex_line();
        let re_ref = GedParser::regex_ref();
        let contents = reader.lines()
            .filter_map(|l| l.ok())
            .filter_map(|l| {
                if let Some(caps) = re.captures(&l) {
                    Some(GedLine::Data(
                        caps.name("Level").unwrap().as_str().parse().unwrap(),
                        caps.name("Tag").unwrap().as_str().to_owned(),
                        if let Some(content) = caps.name("Content") { Some(content.as_str().to_owned()) } else { None }
                    ))
                } else {
                    None
                }
            });
        Ok(RecordRegistry::new())
    }
}

pub trait Buildable {
    type BuilderType: Default;

    fn builder() -> Self::BuilderType {
        Default::default()
    }
}

pub trait Builder {
    type BuildableType: Buildable;

    fn build(self) -> Self::BuildableType;
}

#[derive(Default)]
pub struct GedParserBuilder {
    construct: Box<GedParser>
}

impl Builder for GedParserBuilder {
    type BuildableType = GedParser;

    fn build(self) -> Self::BuildableType {
        *self.construct
    }
}
