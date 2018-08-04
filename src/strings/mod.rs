//! String types.

pub(self) use self::name::validate_name_str;
pub use self::name::{is_name_char, is_name_start_char};
pub use self::name::{NameError, NameStr, NameString};
pub use self::ncname::{is_ncname_char, is_ncname_start_char};
pub use self::ncname::{NcnameStr, NcnameString};
pub use self::qname::Qname;

#[macro_use]
mod macros;

mod name;
mod ncname;
mod qname;
