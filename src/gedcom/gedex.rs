use crate::gedcom::{GedLine,Record,ParseError};

type Predicate = dyn Fn(&GedLine) -> bool;

enum State {
    Initial,
    Reference {records: Vec<Record>},
    RecordTag {records: Vec<Record>, level: i32},
    Invalid
}

impl State {
    fn handle_initial(line: &GedLine) -> Self {
        let cond: &Predicate = &|line| {
            let s = String::from("HEAD");
            match line {
                GedLine::Data(0, s, None) => true,
                _ => false
            }
        };
        if cond(line) {
            Self::Reference {records: Default::default()}
        } else {
            Self::Invalid
        }
    }

    fn handle_ref(recs: Vec<Record>, line: &GedLine) -> Self {
        let cond: &Predicate = &|line| {
            if let GedLine::Ref(lvl, _, _, _) = line {
                if *lvl == 0 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if cond(line) {
            Self::RecordTag {records: recs, level: 1}
        } else {
            Self::Invalid
        }
    }

    fn handle_tag(recs: Vec<Record>, lvl: i32, line: &GedLine) -> Self {
        let cond: &Predicate = &|line| {
            false
        };
        if cond(line) {
            Self::RecordTag {records: recs, level: lvl}
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
            Self::Reference {records} => Self::handle_ref(records, line),
            Self::RecordTag {records, level} => Self::handle_tag(records, level, line),
            Self::Invalid => Self::handle_invalid(line)
        }
    }

    pub fn can_advance(&self) -> bool {
        match self {
            Self::Initial | Self::Reference {..} | Self::RecordTag {..} => true,
            Self::Invalid => false
        }
    }

    pub fn successful(&self) -> bool {
        match self {
            Self::Initial | Self::Reference {..} | Self::Invalid => false,
            Self::RecordTag {..} => true
        }
    }

    pub fn fold(self) -> Result<Vec<Record>, ParseError> {
        match self {
            Self::Initial | Self::Invalid => Err(ParseError::Runtime(String::from("Not GEDCOM data."))),
            Self::Reference {records} | Self::RecordTag {records, ..} => Ok(records)
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
    fn new(contents: Vec<&'a GedLine>) -> Self {
        GedEx {
            contents: contents,
            ..Default::default()
        }
    }

    fn fold(self) -> Result<Vec<Record>, ParseError> {
        Ok(self.records)
    }
}
