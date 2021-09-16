#[derive(Debug)]
pub enum Error {
    NotImplemented,
    Unauthorized,
    NotFound,
    AlreadySubmitted,
    ServiceNotBound(&'static str),
    BuilderError(&'static str),
    InitializationError,
}

impl Error {
    pub fn what(&self) -> String {
        let msg: String = match &self {
            Error::NotImplemented => {
                "NotImplemented: method or function not implemented.".to_string()
            }
            Error::Unauthorized => "Unauthorized: did not pass authentication.".to_string(),
            Error::NotFound => "NotFound: could not found requested data.".to_string(),
            Error::ServiceNotBound(msg) => format!(
                "ServiceNotBound: Could not perform operation because '{}' is not bound.",
                msg
            ),
            Error::BuilderError(msg) => format!(
                "BuilderError: {}",
                msg
            ),
            _ => "Unknown: error type not described".to_string(),
        };
        msg
    }
}

pub type Result<Data> = std::result::Result<Data, Error>;

pub trait View {
    fn assemble(&self) -> gtk::Widget;
}

pub trait EventEmitter<T> {
    fn subscribe<TF: Fn(T) + 'static>(&mut self, f: TF);
}
