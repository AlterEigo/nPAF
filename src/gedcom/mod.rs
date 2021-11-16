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
