use std::rc::Rc;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cell::{RefMut, RefCell};
use std::collections::HashMap;
use unicode_bom::Bom;

extern crate regex;
use regex::Regex;

pub type RecordRef = Rc<RefCell<Record>>;
pub type RecordRegistry = HashMap<u64, Rc<RefCell<Record>>>;
pub type ParseResult = Result<RecordRegistry, ParseError>;

#[derive(Default,Debug,Clone)]
pub struct Record {
    pub id: u64,
    pub name: String,
    pub father: Option<Rc<RefCell<Record>>>,
    pub mother: Option<Rc<RefCell<Record>>>,
    pub children: Vec<Rc<RefCell<Record>>>
}

impl Into<RecordRef> for Record {
    fn into(self) -> RecordRef {
        Rc::new(RefCell::new(self))
    }
}

type IOError = std::io::Error;

#[derive(Debug)]
pub enum ParseError {
    IO(IOError),
    Runtime(String)
}

#[derive(Debug,Clone)]
enum GedLine {
    Data(i32, String, Option<String>),
    Ref(i32, String, Option<String>)
}

impl GedLine {
    fn level(&self) -> i32 {
        match &self {
            Self::Data(lvl, _, _) | Self::Ref(lvl, _, _) => *lvl
        }
    }
}

impl From<IOError> for ParseError {
    fn from(o: IOError) -> ParseError {
        ParseError::IO(o)
    }
}

pub trait Parser {
    type FileType;

    fn read_lines<FT>(file: FT) -> (Bom, Vec<String>)
        where FT: std::io::Read
    {
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

    fn parse(&mut self, file: &Self::FileType) -> ParseResult;
}

#[derive(Default)]
pub struct GedParser {

}

impl GedParser {
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

    fn regex_line() -> Regex {
        Regex::new(r"(?x) # Insignificant whitespace mode
                ^
                (?P<Level>[0-9]{1,2})\ *       # Line level
                (?P<Tag>_?[A-Z]{3,5})\ *       # Record tag
                (?P<Content>[^\r\n]+)*         # Either end of line or content
                $
            ").unwrap()
    }

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

impl Buildable for GedParser {
    type BuilderType = GedParserBuilder;
}

impl Parser for GedParser {
    type FileType = std::fs::File;

    fn parse(&mut self, file: &Self::FileType) -> ParseResult {
        println!("Parsing.");
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_line(&mut contents)?;

        let bom: Bom = contents.as_bytes().into();
        match bom {
            Bom::Utf8 => println!("Utf8 BOM found!"),
            _ => return Err(ParseError::Runtime(std::format!("{:?}", bom)))
        };

        let contents: Vec<GedLine> = reader.lines()
            .filter_map(|l| l.ok())
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
