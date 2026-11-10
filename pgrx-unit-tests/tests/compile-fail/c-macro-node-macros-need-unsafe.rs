use pgrx::pg_sys;

// PostgreSQL's node macros paste `T_` onto their type operand, which the generated macros
// take as a Rust type. Reading the tag that the type names needs no `unsafe`, but each
// macro also reads or allocates a node, in an assert-enabled and in a release build.

// `IsA` reads the node's tag through the caller's pointer.
fn is_a_reads_its_node(node: *mut pg_sys::Node) -> bool {
    pg_sys::IsA!(node, pg_sys::Query).is_true()
}

// `castNode` checks the tag in an assert-enabled build. A release build only casts, and
// needs `unsafe` all the same.
fn cast_node_checks_its_node(node: *mut pg_sys::Node) -> *mut pg_sys::Query {
    pg_sys::castNode!(pg_sys::Query, node).get()
}

// `makeNode` allocates in the current memory context and stores the tag in the result.
fn make_node_allocates() -> *mut pg_sys::Query {
    pg_sys::makeNode!(pg_sys::Query).get()
}

// The list macros need `unsafe` too. Their diagnostics name the list functions they reach,
// which a platform's headers decide whether to translate, so `pgrx-pg-sys`'s `node_macros`
// test pins them instead.

fn main() {}
