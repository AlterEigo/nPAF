use crate::gedcom::{GedLine,Record,ParseError};

enum State {
    Initial,
    Reference(Record),
    RecordTag(Record),
    Invalid
}

impl State {
    fn handle_initial(line: &GedLine) -> Self {
        Default::default()
    }

    fn handle_ref(rec: Record, line: &GedLine) -> Self {
        Default::default()
    }

    fn handle_tag(rec: Record, line: &GedLine) -> Self {
        Default::default()
    }

    fn handle_invalid(line: &GedLine) -> Self {
        Default::default()
    }

    pub fn next(self, line: &GedLine) -> Self {
        match self {
            Self::Initial => Self::handle_initial(line),
            Self::Reference(rec) => Self::handle_ref(rec, line),
            Self::RecordTag(rec) => Self::handle_tag(rec, line),
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
