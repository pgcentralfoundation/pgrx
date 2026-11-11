//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check PostgreSQL's node macros, which paste `T_` onto a type operand to name its node tag.
//!
//! The generated macros take that operand as a Rust type and read the tag from a trait that
//! the node types implement. They are the same macros against an assert-enabled and a
//! release PostgreSQL. Each call below sits in its own `unsafe` block, so `unused_unsafe`
//! rejects a build flavor in which one of them needs none.
//!
//! C names one structure `OpExpr`, `DistinctExpr` and `NullIfExpr`, with a tag for each.
//! Rust sees one type, so the name written in the operand selects the tag, as in C.
//!
//! `IsA` only reads its node, so it runs here. The other macros call PostgreSQL, and are
//! used in generic functions that nothing instantiates: those are type-checked and linted
//! like any other, and compile to no code that would need a backend's symbols.
//! `pgrx-unit-tests` runs them in a backend, and the fixtures of `pgrx-c-macros` check that
//! a wrong tag stops an assert-enabled process.

#![cfg(not(docsrs))]
#![deny(unsafe_op_in_unsafe_fn, unused_unsafe)]

use core::mem::MaybeUninit;
use core::ptr::addr_of_mut;
use pgrx_pg_sys as pg;

/// A second Rust name of a node type, which is the same type.
type Statement = pg::Query;

/// `IsA` compares a node's tag with the tag that C names for the type operand.
#[test]
fn is_a_compares_a_node_with_the_tag_of_its_type_operand() {
    let mut query = MaybeUninit::<pg::Query>::zeroed();
    let node = query.as_mut_ptr().cast::<pg::Node>();
    // SAFETY: `node` points to the aligned storage of a `Query`, which starts with its
    // tag as every node does. Only the tag is written.
    unsafe { addr_of_mut!((*node).type_).write(pg::NodeTag::T_Query) };
    // SAFETY: this call and each one below read only the tag written above.
    assert!(unsafe { pg::IsA!(node, pg::Query) });
    // SAFETY: as above.
    assert!(unsafe { pg::IsA!(node, Statement) });
    // SAFETY: as above.
    assert!(!unsafe { pg::IsA!(node, pg::List) });
    // SAFETY: as above.
    assert!(!unsafe { pg::IsA!(node, pg::RangeVar) });
}

/// `nodeTag` yields the tag as the C integer a node stores, signed on MSVC and unsigned
/// elsewhere. `NodeTag` converts from either, and refuses an integer that is no tag.
///
/// This file is not part of the docs.rs build, whose committed bindings gain the conversions
/// only when they are regenerated.
#[test]
fn node_tag_converts_from_the_integer_a_node_stores() {
    let mut node = MaybeUninit::<pg::Node>::uninit();
    let node = node.as_mut_ptr();
    // SAFETY: `node` points to live, aligned storage. Only its tag is written and read, as
    // the C integer it is stored as, so the tag that is no enumerator never becomes a
    // `NodeTag` value.
    unsafe {
        pg::NodeSetTag!(node, pg::NodeTag::T_Query);
        assert_eq!(pg::NodeTag::try_from(pg::nodeTag!(node).get()), Ok(pg::NodeTag::T_Query));
        pg::NodeSetTag!(node, u32::MAX);
        assert!(pg::NodeTag::try_from(pg::nodeTag!(node).get()).is_err());
    }
}

/// Each C name of the structure that `OpExpr`, `DistinctExpr` and `NullIfExpr` share
/// selects its own tag, however the path to it is written.
#[test]
fn is_a_selects_the_tag_of_the_c_name_its_operand_spells() {
    use pg::{DistinctExpr, NullIfExpr, OpExpr};
    let mut expression = MaybeUninit::<pg::OpExpr>::zeroed();
    let node = expression.as_mut_ptr().cast::<pg::Node>();
    for (tag, expected) in [
        (pg::NodeTag::T_OpExpr, [true, false, false]),
        (pg::NodeTag::T_DistinctExpr, [false, true, false]),
        (pg::NodeTag::T_NullIfExpr, [false, false, true]),
    ] {
        // SAFETY: `node` points to the aligned storage of an `OpExpr`, which starts with
        // its tag as every node does. Only the tag is written.
        unsafe { addr_of_mut!((*node).type_).write(tag) };
        // SAFETY: each call in this loop reads only the tag written above.
        let bare = unsafe {
            [pg::IsA!(node, OpExpr), pg::IsA!(node, DistinctExpr), pg::IsA!(node, NullIfExpr)]
        };
        assert_eq!(bare, expected, "{tag:?}");
        // SAFETY: as above.
        let qualified = unsafe {
            [
                pg::IsA!(node, pg::OpExpr,),
                pg::IsA!(node, ::pgrx_pg_sys::DistinctExpr),
                pg::IsA!(node, pgrx_pg_sys::NullIfExpr,),
            ]
        };
        assert_eq!(qualified, expected, "{tag:?}");
        // SAFETY: as above.
        assert!(!unsafe { pg::IsA!(node, pg::Query) }, "{tag:?}");
    }
}

/// `castNode`, `makeNode` and the list macros take the same names and yield a pointer to
/// the one structure they share.
///
/// # Safety
/// The caller runs on a backend thread with a current memory context. `node` is null or
/// points to a node whose tag is `T_DistinctExpr`, `cell` holds a pointer to a node whose
/// tag is `T_NullIfExpr`, and `list` starts with a pointer to a node whose tag is
/// `T_DistinctExpr`.
#[allow(dead_code)]
unsafe fn shared_names_reach_every_node_macro<Uninstantiated>(
    node: *mut pg::Node,
    list: *mut pg::List,
    cell: *mut pg::ListCell,
) -> [*mut pg::OpExpr; 6] {
    [
        // SAFETY: the caller provides the node this check reads.
        unsafe { pg::castNode!(pg::DistinctExpr, node) }.get(),
        // SAFETY: the caller provides the cell and the node it points to.
        unsafe { pg::lfirst_node!(pg::NullIfExpr, cell) }.get(),
        // SAFETY: the caller provides a list that starts with such a node.
        unsafe { pg::linitial_node!(::pgrx_pg_sys::DistinctExpr, list) }.get(),
        // SAFETY: the caller provides the backend in which `newNode` allocates, for this
        // call and for each one below. The structure is larger than the tag stored in it.
        unsafe { pg::makeNode!(pg::OpExpr) }.get(),
        // SAFETY: as above.
        unsafe { pg::makeNode!(pg::DistinctExpr) }.get(),
        // SAFETY: as above.
        unsafe { pg::makeNode!(pg::NullIfExpr,) }.get(),
    ]
}

/// `castNode` and the list macros built on it yield a pointer to their type operand. An
/// assert-enabled PostgreSQL first checks the node's tag in `castNodeImpl`, so each call
/// needs `unsafe` in both builds, although a release build only casts.
///
/// # Safety
/// The caller runs on a backend thread. `node` is null or points to a `Query`, `cell` holds
/// a pointer to a `RangeVar`, and `list` holds pointers to at least four `RangeVar` nodes.
#[allow(dead_code)]
unsafe fn casts_check_the_tag_in_every_build<Uninstantiated>(
    node: *mut pg::Node,
    list: *mut pg::List,
    cell: *mut pg::ListCell,
) -> [*mut pg::RangeVar; 7] {
    // SAFETY: the caller provides the node this check reads.
    let _: *mut pg::Query = unsafe { pg::castNode!(pg::Query, node) }.get();
    [
        // SAFETY: the caller provides the cell and the node it points to.
        unsafe { pg::lfirst_node!(pg::RangeVar, cell) }.get(),
        // SAFETY: the caller provides a list with at least four such nodes, for this call
        // and for each one below.
        unsafe { pg::linitial_node!(pg::RangeVar, list) }.get(),
        // SAFETY: as above.
        unsafe { pg::lsecond_node!(pg::RangeVar, list) }.get(),
        // SAFETY: as above.
        unsafe { pg::lthird_node!(pg::RangeVar, list) }.get(),
        // SAFETY: as above.
        unsafe { pg::lfourth_node!(pg::RangeVar, list) }.get(),
        // SAFETY: as above.
        unsafe { pg::llast_node!(pg::RangeVar, list) }.get(),
        // SAFETY: as above.
        unsafe { pg::list_nth_node!(pg::RangeVar, list, 3_i32) }.get(),
    ]
}

/// `makeNode` allocates a node of its type operand and stores that type's tag in it, so it
/// needs `unsafe` in both builds.
///
/// PostgreSQL 15 and 16 define `newNode`, which `makeNode` expands, as a macro whose form
/// depends on the compiler: a GNU statement expression for GCC and Clang, and otherwise a
/// comma expression that keeps the node in the global `newNodeMacroHolder`. PostgreSQL 17
/// made it an inline function. Every form gives the same macro.
///
/// # Safety
/// The caller runs on a backend thread with a current memory context.
#[allow(dead_code)]
unsafe fn make_node_needs_unsafe_in_every_build<Uninstantiated>() -> *mut pg::Query {
    // SAFETY: the caller provides the backend in which `newNode` allocates. A `Query` is
    // larger than the tag that `newNode` stores in it.
    unsafe { pg::makeNode!(pg::Query) }.get()
}
