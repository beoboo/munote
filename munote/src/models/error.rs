use nom::error::{ErrorKind, ParseError};

#[derive(Debug)]
pub enum NoteError<I> {
    TagId,
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for NoteError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        NoteError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}
