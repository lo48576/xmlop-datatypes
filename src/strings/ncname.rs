//! NCName string types.
///
/// See <https://www.w3.org/TR/REC-xml-names/#NT-NCName>.

#[cfg(feature = "nom-4")]
use nom::{self, types::CompleteStr};
use opaque_typedef::{OpaqueTypedef, OpaqueTypedefUnsized};

use strings::NameError;
use strings::{is_name_char, is_name_start_char, validate_name_str};

/// Checks whether the given character is NCName start character.
pub fn is_ncname_start_char(c: char) -> bool {
    c != ':' && is_name_start_char(c)
}

/// Checks whether the given character is NCName character.
pub fn is_ncname_char(c: char) -> bool {
    c != ':' && is_name_char(c)
}

/// Validates the given string as `Name`.
fn validate_ncname_str<S: AsRef<str>>(s: S) -> Result<S, NameError> {
    let s = validate_name_str(s)?;
    if let Some(pos) = s.as_ref().find(':') {
        return Err(NameError::InvalidNameChar(pos, ':'));
    }
    Ok(s)
}

define_custom_string! {
    borrowed NcnameStr {
        /// Borrowed NCName, name string without colon (`:`).
        ///
        /// See <https://www.w3.org/TR/REC-xml-names/#NT-NCName>.
        #[opaque_typedef(
            validation(
                validator = "validate_ncname_str",
                error_type = "NameError",
                error_msg = "Failed to create `NcnameStr`"
            )
        )]
    }
    owned NcnameString {
        /// Owned NCName, name string without colon (`:`).
        ///
        /// See <https://www.w3.org/TR/REC-xml-names/#NT-NCName>.
        #[opaque_typedef(
            deref(
                target = "NcnameStr",
                deref = "NcnameStr::from_str_unchecked_implicitly_unsafe"
            )
        )]
        #[opaque_typedef(
            validation(
                validator = "validate_ncname_str",
                error_type = "NameError",
                error_msg = "Failed to create `NcnameString`"
            )
        )]
    }
    extra_impl { str_cmp }
}

impl NcnameStr {
    /// Creates a new `NcnameStr`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xmlop_datatypes::strings::{NcnameStr, NameError};
    /// # fn run() -> Result<(), NameError> {
    /// let s1 = "foo-bar";
    /// let s2 = NcnameStr::new(s1)?;
    /// assert_eq!(s1, s2);
    ///
    /// assert!(NcnameStr::new("contains\"doublequote").is_err());
    /// assert!(NcnameStr::new("contains:colon").is_err());
    /// # Ok(())
    /// # }
    /// # run().expect("Should never fail");
    /// ```
    pub fn new(s: &str) -> Result<&NcnameStr, NameError> {
        <Self as OpaqueTypedefUnsized>::try_from_inner(s)
    }

    /// Creates a new `NcnameStr` from the given string without validation.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the string passed
    /// to it is `NCName` (defined in XML namespaces spec).
    /// If this constraint is violated, undefined behavior results, as the rest
    /// of Rust assumes that `&NcnameStr` has surely `NCName` string.
    ///
    /// So, the argument should fulfill:
    ///
    /// * it is XML `Name`, and
    /// * it does not contain colons (`:`).
    pub unsafe fn from_str_unchecked(s: &str) -> &Self {
        // It is caller's responsibility to ensure that this is safe.
        <Self as OpaqueTypedefUnsized>::from_inner_unchecked(s)
    }
}

#[cfg(feature = "nom-4")]
#[allow(missing_docs)]
impl NcnameStr {
    named!(
        pub nom_parse<CompleteStr, &Self>,
        map!(
            delimited!(
                verify!(peek!(nom::anychar), is_ncname_start_char),
                take_while1!(is_ncname_char),
                alt!(
                    eof!() => { |_| () } |
                    peek!(nom::anychar) => { |_| () }
                )
            ),
            |s| {
                Self::new(*s).unwrap_or_else(|e| {
                    panic!(
                        "Parser is inconsistent with validator of `NcnameStr`: {}",
                        e
                    )
                })
            }
        )
    );
}

impl NcnameString {
    /// Creates a new `NcnameString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xmlop_datatypes::strings::{NcnameString, NameError};
    /// # fn run() -> Result<(), NameError> {
    /// let s1 = "foo-bar".to_owned();
    /// let s2 = NcnameString::new(s1.clone())?;
    /// assert_eq!(s1, s2);
    ///
    /// assert!(NcnameString::new("contains\"doublequote".to_owned()).is_err());
    /// assert!(NcnameString::new("contains:colon".to_owned()).is_err());
    /// # Ok(())
    /// # }
    /// # run().expect("Should never fail");
    /// ```
    pub fn new(s: String) -> Result<Self, NameError> {
        <Self as OpaqueTypedef>::try_from_inner(s)
    }

    /// Creates a new `NcnameString` from the given string without validation.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the string passed
    /// to it is `NCName` (defined in XML namespaces spec).
    /// If this constraint is violated, undefined behavior results, as the rest
    /// of Rust assumes that `NcnameString` has surely `NCName` string.
    ///
    /// So, the argument should fulfill:
    ///
    /// * it is XML `Name`, and
    /// * it does not contain colons (`:`).
    pub unsafe fn new_unchecked(s: String) -> Self {
        <Self as OpaqueTypedef>::from_inner_unchecked(s)
    }

    /// Returns [`&NcnameStr`][`NcnameStr`] slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xmlop_datatypes::strings::{NcnameStr, NcnameString, NameError};
    /// # fn run() -> Result<(), NameError> {
    /// let s = NcnameString::new("foo-bar".to_owned())?;
    /// let _: &NcnameStr = s.as_name_str();
    /// # Ok(())
    /// # }
    /// # run().expect("Should never fail");
    /// ```
    pub fn as_name_str(&self) -> &NcnameStr {
        self.as_ref()
    }
}

#[cfg(feature = "nom-4")]
#[allow(missing_docs)]
impl NcnameString {
    named!(
        pub nom_parse<CompleteStr, Self>,
        map!(
            NcnameStr::nom_parse,
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
    fn parse_ncname() {
        let s = NcnameStr::new("foo-bar").expect("Should never fail");
        let res = NcnameStr::nom_parse("foo-bar  ".into());
        assert_eq!(res, Ok(("  ".into(), s)));

        let s = NcnameStr::new("foo").expect("Should never fail");
        let res = NcnameStr::nom_parse("foo:bar".into());
        assert_eq!(res, Ok((":bar".into(), s)));

        let res = NcnameStr::nom_parse(" foo".into());
        assert_eq!(
            res,
            Err(Err::Error(error_position!(
                " foo".into(),
                ErrorKind::Verify
            )))
        );
    }
}
