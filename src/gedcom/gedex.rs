use crate::gedcom::{GedLine,Record,ParseError};

type Predicate = dyn Fn(&GedLine) -> bool;

enum State {
    Initial,
    Reference(Vec<Record>),
    RecordTag(Vec<Record>),
    Invalid
}

impl State {
    fn handle_initial(line: &GedLine) -> Self {
        let cond: &Predicate = &|line| {
            false
        };
        if cond(line) {
            Self::Reference(Default::default())
        } else {
            Self::Invalid
        }
    }

    fn handle_ref(recs: Vec<Record>, line: &GedLine) -> Self {
        let cond: &Predicate = &|line| {
            false
        };
        if cond(line) {
            Self::RecordTag(recs)
        } else {
            Self::Invalid
        }
    }

    fn handle_tag(recs: Vec<Record>, line: &GedLine) -> Self {
        let cond: &Predicate = &|line| {
            false
        };
        if cond(line) {
            Self::RecordTag(recs)
        } else {
            Self::Invalid
        }
    }

    fn handle_invalid(line: &GedLine) -> Self {
        Self::Invalid
    }

    pub fn next(self, line: &GedLine) -> Self {
        match self {
            Self::Initial => Self::handle_initial(line),
            Self::Reference(recs) => Self::handle_ref(recs, line),
            Self::RecordTag(recs) => Self::handle_tag(recs, line),
            Self::Invalid => Self::handle_invalid(line)
        }
    }

    pub fn can_advance(&self) -> bool {
        match self {
            Self::Initial | Self::Reference(_) | Self::RecordTag(_) => true,
            Self::Invalid => false
        }
    }

    pub fn successful(&self) -> bool {
        match self {
            Self::Initial | Self::Reference(_) | Self::Invalid => false,
            Self::RecordTag(_) => true
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Initial
    }
}

#[derive(Default)]
pub struct GedEx<'a> {
    state: State,
    contents: Vec<&'a GedLine>,
    records: Vec<Record>
}

impl<'a> GedEx<'a> {
    pub fn new(contents: Vec<&'a GedLine>) -> Self {
        GedEx {
            contents: contents,
            ..Default::default()
        }
    }

    pub fn fold(self) -> Result<Vec<Record>, ParseError> {
        Ok(self.records)
    }
}
