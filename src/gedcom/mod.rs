//! This module contains all the tools needed for
//! parsing ged files.

use std::rc::Rc;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cell::{RefMut, RefCell};
use std::collections::HashMap;
use unicode_bom::Bom;

extern crate regex;
pub mod gedex;
use regex::Regex;

/// Smart pointer to a record. Its implemented
/// via the combination of `Rc<RefCell<...>>` so
/// a record could have multiple owners and to be
/// mutable.
/// > Dunno how to use it yet tho :/
pub type RecordRef = Rc<RefCell<Record>>;

/// Structure that contains all the records and
/// assigns each and every one a unique hash, so
/// it could be accessed directly without the need
/// of iterating through the whole record tree.
pub type RecordRegistry = HashMap<u64, Rc<RefCell<Record>>>;

pub type RecordVec = Vec<Record>;
pub type RecordRc = Rc<RefCell<Record>>;

/// Standard result alias
pub type ParseResult = Result<RecordRegistry, ParseError>;

/// Main record tree node containing all the data
/// about a person or a GED data chunk.
/// > Don't know yet what it has to contain,
/// > so it's under heavy developpment.
#[derive(Default,Debug,Clone)]
pub struct Record {
    pub rtype: String,
    pub id: u64,
    pub name: String,
    pub father: Option<RecordRc>,
    pub mother: Option<RecordRc>,
    pub children: Vec<RecordRc>
}

/// Converter from `Record` to `RecordRef`.
/// Allows to crate smart pointers with a
/// call to `Record::into::<RecordRef>()`
impl Into<RecordRef> for Record {
    fn into(self) -> RecordRef {
        Rc::new(RefCell::new(self))
    }
}

/// Convenient alias for `std::io::Error`
type IOError = std::io::Error;

/// Module's error convenient wrapper
#[derive(Debug)]
pub enum ParseError {
    IO(IOError),
    Runtime(String)
}

/// An enumeration that grants the interpretation
/// of a GED data line. Mainly used while parsing
/// the GED file. Allows to quickly access the level
/// of a line and/or its tag with associated data (if any)
/// without the need of mandatory parsing.
/// > This should become a fully qualified data
/// > structure with a type associated to it.
#[derive(Debug,Clone)]
enum GedLine {
    Data(i32, String, Option<String>),
    Ref(i32, String, u64, Option<String>)
}

impl GedLine {
    /// Helper function that returns the line's
    /// record level directly, without the need
    /// of matching the enum manually.
    /// > The enum may become a struct in the future so
    /// > this function might disappear as well.
    fn level(&self) -> i32 {
        match &self {
            Self::Data(lvl, _, _) | Self::Ref(lvl, _, _, _) => *lvl
        }
    }
}

/// Convenient converter from [IOError](std::io::Error) for
/// [ParseError](ParseError)
impl From<IOError> for ParseError {
    fn from(o: IOError) -> ParseError {
        ParseError::IO(o)
    }
}

/// Generic trait made to represent any entity capable of
/// parsing a file to transform it into a [record tree](RecordRegistry)
pub trait Parser {
    type FileType;

    /// This method reads a file, extracts a BOM mark if
    /// it finds one and returns the file contents
    fn read_lines(file: &std::fs::File) -> (Bom, Vec<String>) {
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        if let Err(_) = reader.read_line(&mut first_line) {
            return (Bom::Null, vec![])
        };

        let bom: Bom = first_line.as_bytes().into();
        let mut content: Vec<String> = match bom {
            Bom::Null => vec![first_line],
            _ => vec![first_line[(bom.len() - 1)..].to_owned()],
        };
        let mut rest: Vec<String> = reader.lines()
            .filter_map(|x| x.ok())
            .collect();
        content.append(&mut rest);
        (bom, content)
    }

    /// Main parsing method that all the descendants have to
    /// implement
    fn parse(&mut self, file: &Self::FileType) -> ParseResult;
}

/// Specialized structure for GED parser containing
/// all the data associated to the parser
#[derive(Default)]
pub struct GedParser {

}

impl GedParser {
    fn classic_parse(&mut self, file: &std::fs::File) -> ParseResult {
        Err(ParseError::Runtime(String::from("Not implemented.")))
    }
}

impl Buildable for GedParser {
    type BuilderType = GedParserBuilder;
}

impl Parser for GedParser {
    type FileType = std::fs::File;

    fn parse(&mut self, file: &Self::FileType) -> ParseResult {
        self.classic_parse(file)
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
