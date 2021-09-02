use std::{error::Error as StdError, fmt};

pub struct Error {
    kind: Kind,
}

pub(crate) enum Kind {
    IDduplicate,
    TypeNotFound,
    HandleFailed,
}

impl Error {
    pub(crate) fn new(kind: Kind) -> Self {
        Self {
            kind,
        }
    }

    fn description(&self) -> &str {
        match &self.kind {
            Kind::IDduplicate => "id already exists",
            Kind::TypeNotFound => "type not found",
            Kind::HandleFailed => "failed when handling",
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_tuple("rbi::rvps_handler::Error");

        f.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}
