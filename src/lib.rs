use idna;

mod cookie;
pub use crate::cookie::Error as CookieError;
pub use crate::cookie::{Cookie, CookieResult};
mod cookie_domain;
mod cookie_expiration;
mod cookie_path;
mod cookie_store;
pub use crate::cookie_store::CookieStore;
mod utils;

#[derive(Debug)]
pub struct IdnaErrors(idna::uts46::Errors);

impl std::fmt::Display for IdnaErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "IDNA errors: {:#?}", self.0)
    }
}

impl std::error::Error for IdnaErrors {}

impl From<idna::uts46::Errors> for IdnaErrors {
    fn from(e: idna::uts46::Errors) -> Self {
        IdnaErrors(e)
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
