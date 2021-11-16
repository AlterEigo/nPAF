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

impl Buildable<GedParser> for GedParser {}

impl Parser for GedParser {
    type FileType = std::fs::File;

    fn parse(&mut self, file: &Self::FileType) -> RecordRegistry {
        RecordRegistry::new()
    }
}

pub trait Buildable<T: Default> {
    fn builder() -> Builder<T> {
        Default::default()
    }
}

pub struct Builder<T> {
    construct: Box<T>
}

impl<T> Default for Builder<T> {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Builder<GedParser> {
    fn build(self) -> GedParser {
        *self.construct
    }
}
