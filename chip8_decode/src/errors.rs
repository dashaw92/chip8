use crate::instructions::DecodeErr;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Error {
    InstrErr(DecodeErr),
}