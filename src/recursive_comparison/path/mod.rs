use crate::std::borrow::Cow;
use crate::std::fmt;
use crate::std::fmt::{Debug, Display};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Path<'a>(Vec<Cow<'a, str>>);

impl<'a> Path<'a> {
    pub const SEPARATOR: char = '.';

    pub fn new(field_path: &'a str) -> Self {
        Self::from(field_path)
    }

    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn segments(&self) -> &[Cow<'a, str>] {
        &self.0
    }

    pub fn append(&self, field_name: impl Into<Cow<'a, str>>) -> Self {
        let mut path = self.0.clone();
        path.push(field_name.into());
        Self(path)
    }

    pub fn starts_with(&self, other: &Self) -> bool {
        let other_len = other.0.len();
        let self_len = self.0.len();
        if other_len == 0 {
            return self_len == 0;
        }
        self_len >= other_len && self.0[..other_len] == other.0
    }
}

impl Debug for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for field_name in &self.0 {
            if first {
                first = false;
                write!(f, "{field_name}")?;
            } else {
                write!(f, "{}{field_name}", Self::SEPARATOR)?;
            }
        }
        Ok(())
    }
}

impl<'a> From<&'a str> for Path<'a> {
    fn from(value: &'a str) -> Self {
        let field_names = value
            .split(Path::SEPARATOR)
            .filter_map(|field_name| {
                if field_name.is_empty() {
                    None
                } else {
                    Some(Cow::Borrowed(field_name))
                }
            })
            .collect::<Vec<_>>();
        Self(field_names)
    }
}

impl From<String> for Path<'_> {
    fn from(field_path: String) -> Self {
        let field_names = field_path
            .split(Path::SEPARATOR)
            .filter_map(|field_name| {
                if field_name.is_empty() {
                    None
                } else {
                    Some(Cow::Owned(field_name.to_owned()))
                }
            })
            .collect::<Vec<_>>();
        Path(field_names)
    }
}

#[cfg(test)]
mod tests;
