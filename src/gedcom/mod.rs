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

impl Buildable for GedParser {
    type BuilderType = GedParserBuilder;
}

impl Parser for GedParser {
    type FileType = std::fs::File;

    fn parse(&mut self, file: &Self::FileType) -> ParseResult {
        let mut reader = BufReader::new(file);
        let re = Regex::new(r"(?x) # Insignificant whitespace mode
            ^
            (?P<Level>\d{1,2})\s        # Line level
            (?P<Tag>[A-Z]{4})\s*?       # Record tag
            (?P<Content>[^\r\n]*)+?     # Record content
            $
        ").unwrap();
        let contents = reader.lines()
            .map(|l| l.unwrap())
            .filter(|l| !re.is_match(l))
            .inspect(|l| println!("Unmatched: '{}'", l));
        let unmatched: Vec<String> = contents.collect();
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
