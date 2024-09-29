use std::{
    error::Error,
    fmt::{self, Display},
    marker::PhantomData,
    num::ParseIntError,
    ops::Deref,
    str::FromStr,
};

#[derive(Debug)]
pub struct Id<T: ?Sized>(usize, PhantomData<T>);

impl<T> FromStr for Id<T> {
    type Err = ParseIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(id) => Ok(Self(id, PhantomData)),
            Err(err) => Err(ParseIdError(err)),
        }
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<T> Copy for Id<T> {}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<usize> for Id<T> {
    fn from(value: usize) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> Deref for Id<T> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[derive(Debug)]
pub struct ParseIdError(ParseIntError);

impl fmt::Display for ParseIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cannot parse year: {}", self.0)
    }
}

impl Error for ParseIdError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}
