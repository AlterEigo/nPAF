use crate::gedcom::{GedLine,Record,ParseError};
use std::convert::{TryInto, TryFrom};
use std::io::{BufReader, BufRead};

extern crate regex;
use regex::Regex;

type Predicate = dyn Fn(&GedLine) -> bool;
type TagStack = Vec<Tag>;

#[derive(Default,Clone,Debug)]
struct Tag {
    name: String,
    content: Option<String>,
    nested: Vec<Tag>
}

impl Tag {
    fn new(name: &str, content: Option<&str>) -> Self {
        Tag {
            name: String::from(name),
            content: content.map(|x| String::from(x)),
            ..Default::default()
        }
    }

    fn nest(self, child: Tag) -> Self {
        Self {
            nested: [&self.nested[..], &[child]].concat(),
            ..self
        }
    }
}

#[derive(Debug)]
enum State {
    Initial,
    Reference {sequence: Vec<Tag>},
    RecordTag {sequence: Vec<Tag>, stack: TagStack},
    Invalid
}

fn fold_stack(mut stack: TagStack) -> Option<Tag> {
    let mut stack = stack.into_iter().rev();
    match &stack.len() {
        0 => None,
        1 => stack.next(),
        _ => {
            let last = stack.next().unwrap();
            Some(stack.fold(last, |prev, current| current.nest(prev)))
        }
    }
}

fn fold_stack_lvl<'a>(stack: &'a mut TagStack, pos: usize) -> &'a mut TagStack {
    let folded = fold_stack(stack.drain(pos..).collect());
    if let Some(x) = folded {
        stack.push(x);
    }
    stack
}

impl State {

    fn advance_initial(self, data: GedLine) -> Self {
        match data {
            GedLine::Data(lvl, tag, content) => {
                if lvl != 0 || tag != "HEAD" || content != None {
                    return Self::Invalid;
                }
                let ntag = Tag::new("HEAD", None);
                Self::RecordTag {sequence: Default::default(), stack: vec!(ntag)}
            },
            _ => State::Invalid
        }
    }

    fn concat(s1: &str, s2: &str) -> String {
        [&s1[..], &s2[..]].concat()
    }

    fn advance_ref(self, data: GedLine) -> Self {
        let sequence: Vec<Tag> = match self {
            State::Reference {sequence: seq, ..} => seq,
            _ => panic!("Unexpected state.")
        };
        match data {
            GedLine::Ref(0, rtype, rid, content) => {
                let stack: TagStack = vec!(
                    Tag {
                        name: Self::concat(&rtype, &rid.to_string()),
                        content: content,
                        ..Default::default()
                    }
                );
                Self::RecordTag {sequence: sequence, stack: stack}
            },
            _ => Self::Invalid
        }
    }

    fn advance_ref_or_tag(self, data: GedLine) -> Self {
        let (mut sequence, mut stack) = match self {
            Self::RecordTag {sequence: v1, stack: v2, ..} => (v1, v2),
            _ => panic!("Unexpected state.")
        };
        match data {
            GedLine::Ref(..) => Self::advance_ref(State::Reference {
                sequence: [&sequence[..], &[fold_stack(stack).unwrap()]].concat()
            }, data),
            GedLine::Data(level, tag, content) => {
                let ntag = Tag {
                    name: tag,
                    content: content,
                    ..Default::default()
                };
                let level = usize::from(level);
                let (last_tag, last_level) = (stack.last().unwrap(), stack.len() - 1);
                if level == last_level {
                    fold_stack_lvl(&mut stack, level - 1);
                    stack.push(ntag);
                    Self::RecordTag {sequence: sequence, stack: stack}
                } else if level == (last_level + 1) {
                    stack.push(ntag);
                    Self::RecordTag {sequence: sequence, stack: stack}
                } else if level < last_level {
                    fold_stack_lvl(&mut stack, level - 1);
                    stack.push(ntag);
                    Self::RecordTag {sequence: sequence, stack: stack}
                } else {
                    Self::Invalid
                }
            },
            _ => State::Invalid
        }
    }

    pub fn next(self, line: GedLine) -> Self {
        match self {
            Self::Initial => self.advance_initial(line),
            Self::Reference {..} => self.advance_ref(line),
            Self::RecordTag {..} => self.advance_ref_or_tag(line),
            Self::Invalid => self
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
            Self::RecordTag {sequence: mut seq, stack: stack} => {
                seq.push(
                    fold_stack(stack).unwrap()
                );
                println!("SEQUENCE: {:#?}", seq);
            },
            _ => ()
        };
        Err(
            ParseError::Runtime(
                String::from("Not implemented.")
            )
        )
    }
}

impl Default for State {
    fn default() -> Self {
        State::Initial
    }
}

#[derive(Default)]
pub struct GedEx {
    contents: Vec<String>
}

impl GedEx {
    pub fn new(contents: Vec<String>) -> Self {
        GedEx {
            contents: contents,
            ..Default::default()
        }
    }

    pub fn parse(self) -> Result<Vec<Record>, ParseError> {
        let records = self.contents.into_iter()
            .filter_map(|line| Self::parse_line(&line))
            .fold(State::Initial, |state, line| {
                state.next(line)
            })
            .fold()?;
        Ok(records)
    }

    /// Method allowing to count all the lines that can't
    /// be parsed by the `parse` method.
    pub fn count_unparsed(file: &std::fs::File) -> i64 {
        let reader = BufReader::new(file);
        let re = Self::regex_line();
        let re_ref = Self::regex_ref();
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
                caps.name("Content").map(|s| s.as_str().to_owned())
            ))
        } else if let Some(caps) = r_ref.captures(&line) {
            Some(GedLine::Ref(
                caps.name("Level").unwrap().as_str().parse().unwrap(),
                caps.name("Type").unwrap().as_str().to_owned(),
                caps.name("Number").unwrap().as_str().parse().unwrap(),
                caps.name("Content").map(|s| s.as_str().to_owned())
                // if let Some(content) = caps.name("Content") { Some(content.as_str().to_owned()) } else { None }
            ))
        } else {
            None
        }
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
                (?P<Level>[0-9]{1,2})\ *               # Line level
                @(?P<Type>[A-Z]+)(?P<Number>\d+)@\ *   # Record tag
                (?P<Content>[^\r\n]+)?                 # Either end of line or content
                $
            ").unwrap()
    }
}
