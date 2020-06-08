#[cfg(feature = "pg10")]
mod pg10;

#[cfg(feature = "pg11")]
mod pg11;

#[cfg(feature = "pg12")]
mod pg12;

#[cfg(feature = "pg10")]
pub use pg10::*;

#[cfg(feature = "pg11")]
pub use pg11::*;

#[cfg(feature = "pg12")]
pub use pg12::*;

use crate::{pg_sys, PgBox};

/// #define IsA(nodeptr,_type_)            (nodeTag(nodeptr) == T_##_type_)
#[allow(clippy::not_unsafe_ptr_arg_deref)] // ok b/c we check that nodeptr isn't null
#[inline]
pub fn is_a(nodeptr: *mut pg_sys::Node, tag: pg_sys::NodeTag) -> bool {
    !nodeptr.is_null() && unsafe { nodeptr.as_ref().unwrap().type_ == tag }
}

pub fn node_to_string<'a>(nodeptr: *mut pg_sys::Node) -> Option<&'a str> {
    if nodeptr.is_null() {
        None
    } else {
        let string = unsafe { pg_sys::nodeToString(nodeptr as crate::void_ptr) };
        if string.is_null() {
            None
        } else {
            Some(
                unsafe { std::ffi::CStr::from_ptr(string) }
                    .to_str()
                    .expect("unable to convert Node into a &str"),
            )
        }
    }
}

impl PgNode {
    pub fn is<T>(self, boxed: PgBox<T>) -> bool {
        let node = boxed.as_ptr() as *mut pg_sys::Node;
        let me = self as u32;
        !node.is_null() && unsafe { node.as_ref() }.unwrap().type_ == me
    }
}
