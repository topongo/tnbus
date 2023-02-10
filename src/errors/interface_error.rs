use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum InterfaceError {
    MatchFailed(String)
}

impl Display for InterfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InterfaceError {

}
