use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Record {
    id: u64,
    name: String,
    father: Option<Rc<RefCell<Record>>>,
    mother: Option<Rc<RefCell<Record>>>,
    children: Vec<Rc<RefCell<Record>>>
}

type RecordRegistry = HashMap<u64, Rc<RefCell<Record>>>;

pub trait Parser {
    type FileType;

    fn parse(&mut self, file: &Self::FileType) -> RecordRegistry;
}

#[derive(Default)]
pub struct GedParser {

}

impl Buildable for GedParser {
    type BuilderType = GedParserBuilder;
}

impl Parser for GedParser {
    type FileType = std::fs::File;

    fn parse(&mut self, file: &Self::FileType) -> RecordRegistry {
        RecordRegistry::new()
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
