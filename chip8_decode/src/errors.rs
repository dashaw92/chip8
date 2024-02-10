use crate::instructions::DecodeErr;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug)]
pub enum Error {
    InstrErr(DecodeErr),
}