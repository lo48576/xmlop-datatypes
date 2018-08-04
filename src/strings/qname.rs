//! QName string types.

use std::fmt;

#[cfg(feature = "nom-4")]
use nom::types::CompleteStr;

use strings::{NcnameStr, NcnameString};

/// QName.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Qname {
    /// Prefix part.
    prefix: Option<NcnameString>,
    /// Local part.
    local: NcnameString,
}

impl Qname {
    /// Creates a new `Qname` from the given optional prefix and local part.
    pub fn new<P, L>(prefix: P, local: L) -> Self
    where
        P: Into<Option<NcnameString>>,
        L: Into<NcnameString>,
    {
        Self {
            prefix: prefix.into(),
            local: local.into(),
        }
    }

    /// Creates a new `Qname` from the given prefix and local part.
    pub fn from_prefix_and_local<P, L>(prefix: P, local: L) -> Self
    where
        P: Into<NcnameString>,
        L: Into<NcnameString>,
    {
        Self {
            prefix: Some(prefix.into()),
            local: local.into(),
        }
    }

    /// Creates a new `Qname` from the given local part.
    pub fn from_local<L: Into<NcnameString>>(local: L) -> Self {
        Self {
            prefix: None,
            local: local.into(),
        }
    }

    /// Returns the prefix if available.
    pub fn prefix(&self) -> Option<&NcnameStr> {
        self.prefix.as_ref().map(AsRef::as_ref)
    }

    /// Returns the local part.
    pub fn local(&self) -> &NcnameStr {
        &self.local
    }

    /// Deconstructs `self` into prefix and local part.
    pub fn deconstruct(self) -> (Option<NcnameString>, NcnameString) {
        (self.prefix, self.local)
    }
}

impl fmt::Display for Qname {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(prefix) = self.prefix.as_ref() {
            write!(f, "{}:", prefix)?;
        }
        self.local.fmt(f)
    }
}

#[cfg(feature = "nom-4")]
#[allow(missing_docs)]
impl Qname {
    named!(
        pub nom_parse<CompleteStr, Self>,
        alt!(
            map!(
                separated_pair!(
                    NcnameString::nom_parse,
                    char!(':'),
                    NcnameString::nom_parse
                ),
                |(prefix, local)| Self::from_prefix_and_local(prefix, local)
            ) |
            map!(
                NcnameString::nom_parse,
                Self::from_local
            )
        )
    );
}
