use core::ffi::CStr;
use core::ptr::NonNull;

/// A trait applied to all Postgres `pg_sys::Node` types and subtypes
pub trait PgNode: crate::seal::Sealed + Sized {
    /// [`PgNode`] values that have one of these tags can safely be cast to `Self`
    const CAST_TAGS: &'static [crate::NodeTag];

    /// Try to safely cast node to Self
    fn try_cast<T: PgNode>(node: &T) -> Option<&Self> {
        // This uses `slice::contains` to perform a linear search because the vast majority (>95%)
        // of PgNode have four or fewer tags in `CAST_TAGS`. For small contiguous slices of ints,
        // a linear scan is faster than structured lookups (like binary search or hash sets) due
        // to cache locality, register/instruction efficiency, and compiler autovectorization.
        if Self::CAST_TAGS.contains(&node.node_tag()) {
            Some(unsafe { &*core::ptr::from_ref(node).cast() })
        } else {
            None
        }
    }

    /// Upcast this node to a `pg_sys::Node`
    #[inline]
    fn as_node(&self) -> &crate::Node {
        unsafe { &*core::ptr::from_ref(self).cast() }
    }

    /// Format this node
    ///
    /// # Safety
    ///
    /// While pgrx controls the types for which [`PgNode`] is implemented and only pgrx can implement
    /// [`PgNode`] it cannot control the members of those types and a user could assign any pointer
    /// type member to something invalid that Postgres wouldn't understand.
    ///
    /// Because this function is used by `impl Display` we purposely don't mark it `unsafe`.  The
    /// assumption here, which might be a bad one, is that only intentional misuse would actually
    /// cause undefined behavior.
    #[inline]
    fn display_node(&self) -> String {
        // SAFETY: The trait is pub but this impl is private, and
        // this is only implemented for things known to be "Nodes"
        unsafe { display_node_impl(NonNull::from(self).cast()) }
    }

    #[doc(alias = "IsA")]
    #[inline]
    fn is_a(&self, tag: crate::NodeTag) -> bool {
        self.node_tag() == tag
    }

    #[inline]
    fn node_tag(&self) -> crate::NodeTag {
        self.as_node().type_
    }

    /// Try to safely cast self to T
    #[inline]
    fn try_as<T: PgNode>(&self) -> Option<&T> {
        // Delegate to the associated function which may be specialized in T.
        T::try_cast(self)
    }
}

/// implementation function for `impl Display for $NodeType`
///
/// # Safety
/// Don't use this on anything that doesn't impl PgNode, or the type may be off
#[warn(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn display_node_impl(node: NonNull<crate::Node>) -> String {
    // SAFETY: It's fine to call nodeToString with non-null well-typed pointers,
    // and pg_sys::nodeToString() returns data via palloc, which is never null
    // as Postgres will ERROR rather than giving us a null pointer,
    // and Postgres starts and finishes constructing StringInfos by writing '\0'
    unsafe {
        let node_cstr = crate::nodeToString(node.as_ptr().cast());

        let result = match CStr::from_ptr(node_cstr).to_str() {
            Ok(cstr) => cstr.to_string(),
            Err(e) => format!("<ffi error: {e:?}>"),
        };

        crate::pfree(node_cstr.cast());

        result
    }
}

#[cfg(test)]
mod tests {
    use super::PgNode;

    #[test]
    fn cast_roundtrip() {
        let rt = crate::RangeTblRef { type_: crate::NodeTag::T_RangeTblRef, rtindex: 9 };

        let node = rt.try_as::<crate::Node>().expect("upcast to Node should succeed");
        let next = node.try_as::<crate::RangeTblRef>().expect("downcast to self should succeed");

        assert_eq!(next.type_, crate::NodeTag::T_RangeTblRef);
        assert_eq!(next.rtindex, 9);
    }

    #[test]
    fn inheritance_downcast() {
        let alt_subplan = crate::AlternativeSubPlan {
            xpr: crate::Expr { type_: crate::NodeTag::T_AlternativeSubPlan },
            subplans: std::ptr::null_mut(),
        };

        let node = alt_subplan.as_node(); // infallible upcast
        let expr = node.try_as::<crate::Expr>().expect("downcast to parent of self should succeed");
        assert_eq!(expr.node_tag(), crate::NodeTag::T_AlternativeSubPlan, "tag remains");

        expr.try_as::<crate::AlternativeSubPlan>().expect("downcast to self should succeed");
        assert!(node.try_as::<crate::Var>().is_none(), "downcast to an unrelated type should fail");
    }

    #[test]
    fn inheritance_upcast() {
        let alt_subplan = crate::AlternativeSubPlan {
            xpr: crate::Expr { type_: crate::NodeTag::T_AlternativeSubPlan },
            subplans: std::ptr::null_mut(),
        };

        let expr = alt_subplan.try_as::<crate::Expr>().expect("upcast to a parent should succeed");
        assert_eq!(expr.node_tag(), crate::NodeTag::T_AlternativeSubPlan, "tag remains");

        let node = expr.try_as::<crate::Node>().expect("upcast to Node should succeed");
        assert_eq!(node.node_tag(), crate::NodeTag::T_AlternativeSubPlan, "tag remains");
    }

    // In versions prior to pg15, the `Value` struct is used with tags T_Null, T_Integer, T_Float,
    // T_String, and T_BitString.
    #[cfg(any(feature = "pg13", feature = "pg14"))]
    #[test]
    fn value_struct_cast() {
        let value =
            crate::Value { type_: crate::NodeTag::T_Integer, val: crate::ValUnion { ival: 42 } };

        let node = value.as_node(); // infallible upcast
        assert_eq!(node.node_tag(), crate::NodeTag::T_Integer, "tag remains");

        let next = node.try_as::<crate::Value>().expect("downcast to self should succeed");
        assert_eq!(next.node_tag(), crate::NodeTag::T_Integer, "tag remains");
        assert_eq!(unsafe { next.val.ival }, 42);

        for tag in &[
            crate::NodeTag::T_Float,
            crate::NodeTag::T_String,
            crate::NodeTag::T_BitString,
            crate::NodeTag::T_Null,
        ] {
            let value = crate::Value {
                type_: *tag,
                val: crate::ValUnion { str_: c"something".as_ptr() as *mut _ },
            };

            let node = value.as_node(); // infallible upcast
            assert_eq!(node.node_tag(), *tag, "tag remains");

            let next = node.try_as::<crate::Value>().expect("downcast to self should succeed");
            assert_eq!(next.node_tag(), *tag, "tag remains");
            assert_eq!(unsafe { next.val.str_ }, c"something".as_ptr() as *mut _);
        }
    }

    // Since pg15, Boolean, Integer, Float, String, and BitString are separate node structs.
    // `ValUnion` is a single node that can hold any one of these types. That union should implement
    // PgNode so we can cast to one of the above structs.
    #[cfg(any(
        feature = "pg15",
        feature = "pg16",
        feature = "pg17",
        feature = "pg18",
        feature = "pg19"
    ))]
    #[test]
    fn value_union_cast() {
        let vu = crate::ValUnion {
            sval: crate::String {
                type_: crate::NodeTag::T_String,
                sval: c"something".as_ptr() as *mut _,
            },
        };

        let node = vu.try_as::<crate::Node>().expect("upcast to Node should succeed");
        let next = node.try_as::<crate::String>().expect("downcast to self should succeed");

        assert_eq!(next.type_, crate::NodeTag::T_String);
        assert_eq!(next.sval, c"something".as_ptr() as *mut _);

        assert!(
            node.try_as::<crate::ValUnion>().is_none(),
            "downcast to untagged union should fail"
        );
    }
}
