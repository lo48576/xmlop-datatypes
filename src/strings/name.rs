//! Name string types.
///
/// See <https://www.w3.org/TR/2006/REC-xml11-20060816/#NT-Name>.
use std::error;
use std::fmt;

#[cfg(feature = "nom-4")]
use nom::{self, types::CompleteStr};
use opaque_typedef::{OpaqueTypedef, OpaqueTypedefUnsized};

/// Checks whether the given character is name start character.
///
/// See <https://www.w3.org/TR/2006/REC-xml11-20060816/#NT-NameStartChar>.
pub fn is_name_start_char(c: char) -> bool {
    match c {
        ':'
        | 'A'..='Z'
        | '_'
        | 'a'..='z'
        | '\u{C0}'..='\u{D6}'
        | '\u{D8}'..='\u{F6}'
        | '\u{F8}'..='\u{2FF}'
        | '\u{370}'..='\u{37D}'
        | '\u{37F}'..='\u{1FFF}'
        | '\u{200C}'..='\u{200D}'
        | '\u{2070}'..='\u{218F}'
        | '\u{2C00}'..='\u{2FEF}'
        | '\u{3001}'..='\u{D7FF}'
        | '\u{F900}'..='\u{FDCF}'
        | '\u{FDF0}'..='\u{FFFD}'
        | '\u{10000}'..='\u{EFFFF}' => true,
        _ => false,
    }
}

/// Checks whether the given character is name start character.
///
/// See <https://www.w3.org/TR/2006/REC-xml11-20060816/#NT-NameChar>.
pub fn is_name_char(c: char) -> bool {
    is_name_start_char(c) || match c {
        '-' | '.' | '0'..='9' | '\u{B7}' | '\u{0300}'..='\u{036F}' | '\u{203F}'..='\u{2040}' => {
            true
        },
        _ => false,
    }
}

/// XML name string error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NameError {
    /// Got empty string.
    Empty,
    /// Has invalid character.
    InvalidNameChar(usize, char),
}

impl error::Error for NameError {}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NameError::Empty => f.write_str("XML name string should not be empty"),
            NameError::InvalidNameChar(pos, c) => write!(
                f,
                "Invalid name character at byte position {}: {:?}",
                pos, c
            ),
        }
    }
}

/// Validates the given string as `Name`.
pub(crate) fn validate_name_str<S: AsRef<str>>(s: S) -> Result<S, NameError> {
    if s.as_ref().is_empty() {
        return Err(NameError::Empty);
    }
    {
        let s = s.as_ref();
        assert!(!s.is_empty());
        let mut chars = s.chars().enumerate();
        let (_, head) = chars
            .next()
            .unwrap_or_else(|| unreachable!("Should never fail because the string is empty"));
        if !is_name_start_char(head) {
            return Err(NameError::InvalidNameChar(0, head));
        }
        if let Some((pos, c)) = chars.find(|&(_, c)| !is_name_char(c)) {
            return Err(NameError::InvalidNameChar(pos, c));
        }
    }
    Ok(s)
}

define_custom_string! {
    borrowed NameStr {
        /// Borrowed XML Name.
        ///
        /// See <https://www.w3.org/TR/2006/REC-xml11-20060816/#NT-Name>.
        #[opaque_typedef(
            validation(
                validator = "validate_name_str",
                error_type = "NameError",
                error_msg = "Failed to create `NameStr`"
            )
        )]
    }
    owned NameString {
        /// Owned XML Name.
        ///
        /// See <https://www.w3.org/TR/2006/REC-xml11-20060816/#NT-Name>.
        #[opaque_typedef(
            deref(
                target = "NameStr",
                deref = "NameStr::from_str_unchecked_implicitly_unsafe"
            )
        )]
        #[opaque_typedef(
            validation(
                validator = "validate_name_str",
                error_type = "NameError",
                error_msg = "Failed to create `NameString`"
            )
        )]
    }
    extra_impl { str_cmp }
}

impl NameStr {
    /// Creates a new `NameStr`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xmlop_datatypes::strings::{NameStr, NameError};
    /// # fn run() -> Result<(), NameError> {
    /// let s1 = "foo:bar";
    /// let s2 = NameStr::new(s1)?;
    /// assert_eq!(s1, s2);
    ///
    /// assert!(NameStr::new("contains\"doublequote").is_err());
    /// # Ok(())
    /// # }
    /// # run().expect("Should never fail");
    /// ```
    pub fn new(s: &str) -> Result<&NameStr, NameError> {
        <Self as OpaqueTypedefUnsized>::try_from_inner(s)
    }

    /// Creates a new `NameStr` from the given string without validation.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the string passed
    /// to it is XML `Name` (defined in XML spec).
    /// If this constraint is violated, undefined behavior results, as the rest
    /// of Rust assumes that `&NameStr` has surely XML `Name` string.
    ///
    /// So, the argument should fulfill:
    ///
    /// * it is XML `Name`.
    pub unsafe fn from_str_unchecked(s: &str) -> &Self {
        // It is caller's responsibility to ensure that this is safe.
        <Self as OpaqueTypedefUnsized>::from_inner_unchecked(s)
    }
}

#[cfg(feature = "nom-4")]
#[allow(missing_docs)]
impl NameStr {
    named!(
        pub nom_parse<CompleteStr, &Self>,
        map!(
            preceded!(
                verify!(
                    peek!(nom::anychar),
                    is_name_start_char
                ),
                take_while1!(is_name_char)
            ),
            |s| Self::new(*s).unwrap_or_else(|e| {
                panic!("Parser is inconsistent with validator of `NameStr`: {}", e)
            })
        )
    );
}

impl NameString {
    /// Creates a new `NameString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xmlop_datatypes::strings::{NameString, NameError};
    /// # fn run() -> Result<(), NameError> {
    /// let s1 = "foo:bar".to_owned();
    /// let s2 = NameString::new(s1.clone())?;
    /// assert_eq!(s1, s2);
    ///
    /// assert!(NameString::new("contains\"doublequote".to_owned()).is_err());
    /// # Ok(())
    /// # }
    /// # run().expect("Should never fail");
    /// ```
    pub fn new(s: String) -> Result<Self, NameError> {
        <Self as OpaqueTypedef>::try_from_inner(s)
    }

    /// Creates a new `NameString` from the given string without validation.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the string passed
    /// to it is XML `Name` (defined in XML spec).
    /// If this constraint is violated, undefined behavior results, as the rest
    /// of Rust assumes that `NameString` has surely XML `Name` string.
    ///
    /// So, the argument should fulfill:
    ///
    /// * it is XML `Name`.
    pub unsafe fn new_unchecked(s: String) -> Self {
        <Self as OpaqueTypedef>::from_inner_unchecked(s)
    }

    /// Returns [`&NameStr`][`NameStr`] slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xmlop_datatypes::strings::{NameStr, NameString, NameError};
    /// # fn run() -> Result<(), NameError> {
    /// let s = NameString::new("foo:bar".to_owned())?;
    /// let _: &NameStr = s.as_name_str();
    /// # Ok(())
    /// # }
    /// # run().expect("Should never fail");
    /// ```
    pub fn as_name_str(&self) -> &NameStr {
        self.as_ref()
    }
}

#[cfg(feature = "nom-4")]
#[allow(missing_docs)]
impl NameString {
    named!(
        pub nom_parse<CompleteStr, Self>,
        map!(
            NameStr::nom_parse,
            ToOwned::to_owned
        )
    );
}

#[cfg(feature = "nom-4")]
#[cfg(test)]
mod nom_tests {
    use nom::{Err, ErrorKind};

    use super::*;

    #[test]
    fn parse_xml_name() {
        let s = NameStr::new("foo-bar").expect("Should never fail");
        let res = NameStr::nom_parse("foo-bar  ".into());
        assert_eq!(res, Ok(("  ".into(), s)));

        let s = NameStr::new("foo:bar").expect("Should never fail");
        let res = NameStr::nom_parse("foo:bar  ".into());
        assert_eq!(res, Ok(("  ".into(), s)));

        let res = NameStr::nom_parse(" foo".into());
        assert_eq!(
            res,
            Err(Err::Error(error_position!(
                " foo".into(),
                ErrorKind::Verify
            )))
        );
    }
}
