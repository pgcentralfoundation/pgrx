// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! Helper functions and such for Postgres' various query tree `Node`s

use crate::pg_sys;

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
