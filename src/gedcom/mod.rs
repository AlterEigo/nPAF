//! This module contains all the tools needed for
//! parsing ged files.

use std::rc::Rc;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cell::{RefMut, RefCell};
use std::collections::HashMap;
use unicode_bom::Bom;

extern crate regex;
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

/// Standard result alias
pub type ParseResult = Result<RecordRegistry, ParseError>;

/// Main record tree node containing all the data
/// about a person or a GED data chunk.
/// > Don't know yet what it has to contain,
/// > so it's under heavy developpment.
#[derive(Default,Debug,Clone)]
pub struct Record {
    pub id: u64,
    pub name: String,
    pub father: Option<Rc<RefCell<Record>>>,
    pub mother: Option<Rc<RefCell<Record>>>,
    pub children: Vec<Rc<RefCell<Record>>>
}

trait NdfaState {
    fn advance<'a>(self, line: &'a GedLine) -> Box<dyn NdfaState>;

    fn is_final(&self) -> bool {
        return false;
    }

    fn success(&self) -> bool {
        return false;
    }

    fn fold(self) -> RecordVec;
}

#[derive(Default,Debug,Clone)]
struct EntryState {
    model: RecordVec,
}

#[derive(Default,Debug,Clone)]
struct ReferenceState {
    model: RecordVec
}

#[derive(Default,Debug,Clone)]
struct TagState {
    model: RecordVec,
    level: i32
}

#[derive(Default,Debug,Clone)]
struct FailedState {
}

impl NdfaState for TagState {
    fn advance<'a>(mut self, line: &'a GedLine) -> Box<dyn NdfaState> {
        if let GedLine::Data(lvl, tag, _) = line {

            if *lvl < self.level {
                return Box::new(ReferenceState {
                    model: self.model
                })
            }

            if *lvl == self.level {
                match tag {
                    _ => self.model.push(Record {
                        ..Default::default()
                    })
                }
                return Box::new(self);
            }

            if *lvl == self.level + 1 {
                let mut rec: Record = self.model.pop().unwrap();
                match tag {
                    _ => rec.children.push(Record {
                        ..Default::default()
                    }.into())
                }
                self.model.push(rec);
                return Box::new(TagState {
                    model: self.model,
                    level: self.level + 1
                })
            }
        }
        Box::new(FailedState {})
    }

    fn success(&self) -> bool {
        true
    }

    fn is_final(&self) -> bool {
        true
    }

    fn fold(self) -> RecordVec {
        self.model
    }
}

impl NdfaState for ReferenceState {
    fn advance<'a>(mut self, line: &'a GedLine) -> Box<dyn NdfaState> {
        if let GedLine::Ref(lvl, rid, None) = line {
            if *lvl == 0 {
                let rec: Record = Record {
                    ..Default::default()
                };
                self.model.push(rec);
                return Box::new(TagState {
                    model: self.model,
                    level: 1
                });
            }
        }
        Box::new(FailedState {})
    }

    fn success(&self) -> bool {
        false
    }

    fn is_final(&self) -> bool {
        false
    }

    fn fold(self) -> RecordVec {
        self.model
    }
}

impl NdfaState for EntryState {
    fn advance<'a>(mut self, line: &'a GedLine) -> Box<dyn NdfaState> {
        if let GedLine::Data(lvl, tag, None) = line {
            if *lvl == 0 && *tag == "HEAD" {
                return Box::new(TagState {
                    model: Default::default(),
                    level: 1
                });
            }
        }
        Box::new(FailedState {})
    }

    fn success(&self) -> bool {
        false
    }

    fn is_final(&self) -> bool {
        false
    }

    fn fold(self) -> RecordVec {
        Default::default()
    }
}

impl NdfaState for FailedState {
    fn advance<'a>(mut self, _: &'a GedLine) -> Box<dyn NdfaState> {
        Box::new(self)
    }

    fn is_final(&self) -> bool {
        true
    }

    fn success(&self) -> bool {
        false
    }

    fn fold(self) -> RecordVec {
        Default::default()
    }
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
    Ref(i32, String, Option<String>)
}

impl GedLine {
    /// Helper function that returns the line's
    /// record level directly, without the need
    /// of matching the enum manually.
    /// > The enum may become a struct in the future so
    /// > this function might disappear as well.
    fn level(&self) -> i32 {
        match &self {
            Self::Data(lvl, _, _) | Self::Ref(lvl, _, _) => *lvl
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

    /// Method allowing to count all the lines that can't
    /// be parsed by the `parse` method.
    pub fn count_unparsed(&self, file: &std::fs::File) -> i64 {
        let reader = BufReader::new(file);
        let re = GedParser::regex_line();
        let re_ref = GedParser::regex_ref();
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
                if let Some(content) = caps.name("Content") { Some(content.as_str().to_owned()) } else { None }
            ))
        } else if let Some(caps) = r_ref.captures(&line) {
            Some(GedLine::Ref(
                caps.name("Level").unwrap().as_str().parse().unwrap(),
                caps.name("Tag").unwrap().as_str().to_owned(),
                if let Some(content) = caps.name("Content") { Some(content.as_str().to_owned()) } else { None }
            ))
        } else {
            None
        }
    }

    /// Reading a collection of lines and recursively transform
    /// it into a data record, unifying multiple lines into one
    /// logical entity
    fn read_record<'a>(origin: &'a [GedLine]) -> (&'a [GedLine], Option<Record>)
    {
        match origin.len() {
            0 => return (origin, None),
            1 => return (&origin[1..], Some(Default::default())),
            _ => ()
        };
        let lvl: Vec<i32> = origin[..2]
            .into_iter()
            .map(|val| val.level())
            .collect();
        if lvl[1] <= lvl[0] {
            return (&origin[1..], Some(Default::default()))
        }
        let lvl = lvl[0];
        let mut origin = &origin[1..];
        let mut record: Record = Default::default();

        let mut nlevel = if !origin.is_empty() { origin[0].level() } else { 0 };
        while !origin.is_empty() && nlevel > lvl {
            let (rest, child) = Self::read_record(origin);
            if let None = child {
                break;
            }
            let child = child
                .map(|val| Rc::new(RefCell::new(val)))
                .unwrap();
            record.children.push(child);
            origin = rest;
            nlevel = if !origin.is_empty() { origin[0].level() } else { 0 };
        }
        (origin, Some(record))
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
                (?P<Level>[0-9]{1,2})\ *           # Line level
                @(?P<Tag>[A-Z]+\d+)@\ *            # Record tag
                (?P<Content>[^\r\n]+)?             # Either end of line or content
                $
            ").unwrap()
    }
}

/// 
impl Buildable for GedParser {
    type BuilderType = GedParserBuilder;
}

impl Parser for GedParser {
    type FileType = std::fs::File;

    fn parse(&mut self, file: &Self::FileType) -> ParseResult {
        println!("Parsing.");
        let (_, contents) = Self::read_lines(file);
        let contents: Vec<GedLine> = contents.iter()
            .filter_map(|l| Self::parse_line(&l))
            .collect();
        println!("Contents read, line count: '{}'.", contents.len());
        let mut records: Vec<RecordRef> = Vec::new();
        let mut slice = contents.as_slice();
        loop {
            let (rest, rec) = Self::read_record(slice);
            if let None = rec {
                break;
            }
            println!("{:#?}", rec);
            slice = rest;
        }
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
