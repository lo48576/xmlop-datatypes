//! Macros for string types definition.

/// Implement comparations.
macro_rules! impl_cmp {
    ($Inner:ty, $Lhs:ty, $Rhs:ty) => {
        impl PartialEq<$Rhs> for $Lhs {
            fn eq(&self, rhs: &$Rhs) -> bool {
                AsRef::<$Inner>::as_ref(self).eq(AsRef::<$Inner>::as_ref(rhs))
            }
        }
        impl PartialEq<$Lhs> for $Rhs {
            fn eq(&self, rhs: &$Lhs) -> bool {
                AsRef::<$Inner>::as_ref(self).eq(AsRef::<$Inner>::as_ref(rhs))
            }
        }
        impl PartialOrd<$Rhs> for $Lhs {
            fn partial_cmp(&self, rhs: &$Rhs) -> Option<::std::cmp::Ordering> {
                AsRef::<$Inner>::as_ref(self).partial_cmp(AsRef::<$Inner>::as_ref(rhs))
            }
        }
        impl PartialOrd<$Lhs> for $Rhs {
            fn partial_cmp(&self, rhs: &$Lhs) -> Option<::std::cmp::Ordering> {
                AsRef::<$Inner>::as_ref(self).partial_cmp(AsRef::<$Inner>::as_ref(rhs))
            }
        }
    };
    ($Inner:ty, $Lhs:ty, $Rhs:ty, $($lts:tt)*) => {
        impl<$($lts)*> PartialEq<$Rhs> for $Lhs {
            fn eq(&self, rhs: &$Rhs) -> bool {
                AsRef::<$Inner>::as_ref(self).eq(AsRef::<$Inner>::as_ref(rhs))
            }
        }
        impl<$($lts)*> PartialEq<$Lhs> for $Rhs {
            fn eq(&self, rhs: &$Lhs) -> bool {
                AsRef::<$Inner>::as_ref(self).eq(AsRef::<$Inner>::as_ref(rhs))
            }
        }
        impl<$($lts)*> PartialOrd<$Rhs> for $Lhs {
            fn partial_cmp(&self, rhs: &$Rhs) -> Option<::std::cmp::Ordering> {
                AsRef::<$Inner>::as_ref(self).partial_cmp(AsRef::<$Inner>::as_ref(rhs))
            }
        }
        impl<$($lts)*> PartialOrd<$Lhs> for $Rhs {
            fn partial_cmp(&self, rhs: &$Lhs) -> Option<::std::cmp::Ordering> {
                AsRef::<$Inner>::as_ref(self).partial_cmp(AsRef::<$Inner>::as_ref(rhs))
            }
        }
    };
}

/// Define custom string types and implement basic methods.
macro_rules! define_custom_string {
    (
        borrowed $borrowed:ident { $(#[$borrowed_meta:meta])* }
        owned $owned:ident { $(#[$owned_meta:meta])* }
        $($feature:ident { $($feature_inner:tt)* })*
    ) => {
        define_custom_string!(@borrowed, $borrowed, $owned, $($borrowed_meta)*);
        define_custom_string!(@owned, $borrowed, $owned, $($owned_meta)*);
        define_custom_string!(@cmp, $borrowed, $owned);
        $(
            define_custom_string!(@feature, $borrowed, $owned, $feature { $($feature_inner)* });
        )*
    };
    (@borrowed, $borrowed:ident, $owned:ident, $($meta:meta)*) => {
        // `derive_hash_xor_eq`: clippy lint.
        #[allow(unknown_lints, derive_hash_xor_eq)]
        #[derive(Debug, Eq, Hash, OpaqueTypedefUnsized)]
        #[opaque_typedef(
            derive(
                AsRef(Deref, Self_),
                Deref,
                Display,
                Into(Arc, Box, Rc, Inner),
                PartialEq(Self_, Inner, InnerRev, InnerCow, SelfCow, SelfCowRev),
                PartialOrd(Self_, Inner, InnerRev, InnerCow, SelfCow, SelfCowRev),
                Ord,
            )
        )]
        #[repr(transparent)]
        $(#[$meta])*
        pub struct $borrowed(str);

        impl $borrowed {
            /// Creates a borrowed string from the string slice.
            ///
            /// This is intended to be used by `opaque_typedef`, and it is because this
            /// function is not `unsafe`.
            fn from_str_unchecked_implicitly_unsafe(s: &str) -> &Self {
                unsafe {
                    // It is caller's (`opaque_typedef`'s) responsibility to ensure that
                    // this is safe.
                    <Self as ::opaque_typedef::OpaqueTypedefUnsized>::from_inner_unchecked(s)
                }
            }

            /// Returns a reference to the inner string as `&str`.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl ToOwned for $borrowed {
            type Owned = $owned;

            fn to_owned(&self) -> Self::Owned {
                unsafe {
                    // This is safe because `&self.0` is validated at `self` creation.
                    <$owned as ::opaque_typedef::OpaqueTypedef>::from_inner_unchecked(
                        self.0.to_owned()
                    )
                }
            }
        }

        impl<'a> From<&'a $borrowed> for String {
            fn from(s: &'a $borrowed) -> Self {
                s.as_str().into()
            }
        }
    };
    (@owned, $borrowed:ident, $owned:ident, $($meta:meta)*) => {
        // `derive_hash_xor_eq`: clippy lint.
        #[allow(unknown_lints, derive_hash_xor_eq)]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, OpaqueTypedef)]
        #[opaque_typedef(
            derive(
                AsRef(Deref, Inner),
                Deref,
                Display,
                IntoInner,
                PartialEq(Inner, InnerRev),
                PartialOrd(Inner, InnerRev)
            )
        )]
        $(#[$meta])*
        pub struct $owned(String);

        impl $owned {
            /// Returns a reference to the inner string as `&str`.
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl ::std::borrow::Borrow<$borrowed> for $owned {
            fn borrow(&self) -> &$borrowed {
                self.as_ref()
            }
        }

        impl AsRef<str> for $owned {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl<'a> From<&'a $borrowed> for $owned {
            fn from(s: &'a $borrowed) -> Self {
                s.to_owned()
            }
        }
    };
    (@cmp, $borrowed:ident, $owned:ident) => {
        // $borrowed - $borrowed
        impl_cmp!($borrowed, &'a $borrowed, $borrowed, 'a);
        // $owned - $borrowed
        impl_cmp!($borrowed, $owned, $borrowed);
        impl_cmp!($borrowed, &'a $owned, $borrowed, 'a);
        impl_cmp!($borrowed, $owned, &'a $borrowed, 'a);
        // $owned - $owned
        impl_cmp!($borrowed, &'a $owned, $owned, 'a);
    };
    (@feature, $borrowed:ident, $owned:ident, extra_impl { $($inner:ident),* }) => {
        $(
            define_custom_string!(@extra_impl, $borrowed, $owned, $inner);
        )*
    };
    (@extra_impl, $borrowed:ident, $owned:ident, str_cmp) => {
        // $borrowed - String
        impl_cmp!(str, $borrowed, String);
        impl_cmp!(str, &'a $borrowed, String, 'a);
        impl_cmp!(str, $borrowed, &'a String, 'a);
        // $owned - str
        impl_cmp!(str, $owned, str);
        impl_cmp!(str, &'a $owned, str, 'a);
        impl_cmp!(str, $owned, &'a str, 'a);
    };
    (@extra_impl, $borrowed:ident, $owned:ident, pub_new) => {
        impl $borrowed {
            /// Creates a new string slice.
            pub fn new(s: &str) -> &$borrowed {
                <Self as ::opaque_typedef::OpaqueTypedefUnsized>::from_inner(s)
            }
        }

        impl $owned {
            /// Creates a new owned string.
            pub fn new(s: String) -> $owned {
                <Self as ::opaque_typedef::OpaqueTypedef>::from_inner(s)
            }
        }
    };
}
