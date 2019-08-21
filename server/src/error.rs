
use std::{error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidResources;


impl error::Error for InvalidResources {
    fn description(&self) -> &str {
        "Resources cant be more than 100k"
    }
}


impl fmt::Display for InvalidResources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err: &dyn error::Error = self;
        f.write_str(err.description())
    }
}
