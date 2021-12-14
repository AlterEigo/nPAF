use crate::gedcom::{GedLine,Record,ParseError};

enum State {
    Initial,
    Reference,
    RecordTag,
    Invalid
}

impl State {
    fn handle_initial(line: &GedLine) -> Self {
        Default::default()
    }

    fn handle_ref(line: &GedLine) -> Self {
        Default::default()
    }

    fn handle_tag(line: &GedLine) -> Self {
        Default::default()
    }

    fn handle_invalid(line: &GedLine) -> Self {
        Default::default()
    }

    pub fn next(self, line: &GedLine) -> Self {
        let func: &dyn Fn(&GedLine) -> State = match self {
            Self::Initial => &Self::handle_initial,
            Self::Reference => &Self::handle_ref,
            Self::RecordTag => &Self::handle_tag,
            Self::Invalid => &Self::handle_invalid
        };
        func(line)
    }

    pub fn can_advance(&self) -> bool {
        match self {
            Self::Initial | Self::Reference | Self::RecordTag => true,
            Self::Invalid => false
        }
    }

    pub fn successful(&self) -> bool {
        match self {
            Self::Initial | Self::Reference | Self::Invalid => false,
            Self::RecordTag => true
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
