use std::rc::Rc;
use std::cell::RefCell;

pub struct Record {
    name: String,
    father: Option<Rc<RefCell<Record>>>,
    mother: Option<Rc<RefCell<Record>>>,
    children: Vec<Rc<RefCell<Record>>>
}

pub trait Parser {
    
}
