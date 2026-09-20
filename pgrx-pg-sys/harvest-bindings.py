#!/usr/bin/env python3
"""Harvest C-layout type/const items from a pgrx bindgen file for the pgrust target.

Usage: pgrx-harvest-bindings.py <pg18.rs> <names.list> <out.rs> [--fns <fnnames.list> <out-fns.txt>]

Items (const/type/struct/enum/union/mod + attached impl blocks) named in
names.list are emitted in file order together with the transitive closure of
the items they reference. Function declarations are never harvested: the
pgrust target hand-writes every function. With --fns, the bindgen signatures of
the named functions are dumped to a text file as a reference for the shim.

Lines in names.list: one identifier per line, `#` comments, or `regex:<re>`
to select every item whose name matches the regex.
"""
import re
import sys

ITEM_RE = re.compile(
    r"^pub (?:const|type|struct|enum|union|mod|static) (?P<name>[A-Za-z_][A-Za-z0-9_]*)"
)
IMPL_RE = re.compile(
    r"^impl(?:<[^>]*>)? (?:(?:[A-Za-z_:]+)(?:<[^>]*>)? for )?(?P<name>[A-Za-z_][A-Za-z0-9_]*)"
)
IDENT_RE = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]*)\b")


def parse_items(lines):
    """Return (items: name -> [text chunks in order], order: list of (name, text))."""
    items = {}
    order = []
    attrs = []
    i = 0
    n = len(lines)
    in_extern = False
    while i < n:
        line = lines[i]
        if in_extern:
            if line.startswith("}"):
                in_extern = False
            i += 1
            continue
        if line.startswith("unsafe extern") or line.startswith("extern \"C"):
            in_extern = True
            i += 1
            continue
        if line.startswith("#["):
            attrs.append(line)
            i += 1
            continue
        m = ITEM_RE.match(line) or IMPL_RE.match(line)
        if not m:
            attrs = []
            i += 1
            continue
        name = m.group("name")
        # collect until braces balance and the item ends
        depth = 0
        chunk = list(attrs)
        attrs = []
        j = i
        while j < n:
            l = lines[j]
            chunk.append(l)
            depth += l.count("{") - l.count("}")
            stripped = l.rstrip()
            if depth <= 0 and (stripped.endswith(";") or stripped.endswith("}")):
                break
            j += 1
        text = "\n".join(chunk)
        order.append((name, text))
        items.setdefault(name, []).append(text)
        i = j + 1
    return items, order


def parse_fn_sigs(lines):
    sigs = {}
    in_extern = False
    buf = None
    for line in lines:
        if line.startswith("unsafe extern"):
            in_extern = True
            continue
        if in_extern and line.startswith("}"):
            in_extern = False
            continue
        if not in_extern:
            continue
        s = line.strip()
        if buf is None:
            if s.startswith("pub fn ") or s.startswith("pub static mut "):
                buf = [line]
                if s.endswith(";"):
                    sig = "\n".join(buf)
                    buf = None
                    m = re.search(r"(?:fn|mut) ([A-Za-z_][A-Za-z0-9_]*)", sig)
                    sigs[m.group(1)] = sig
        else:
            buf.append(line)
            if s.endswith(";"):
                sig = "\n".join(buf)
                buf = None
                m = re.search(r"(?:fn|mut) ([A-Za-z_][A-Za-z0-9_]*)", sig)
                sigs[m.group(1)] = sig
    return sigs


def load_names(path):
    exact, regexes = set(), []
    for raw in open(path):
        s = raw.strip()
        if not s or s.startswith("#"):
            continue
        if s.startswith("regex:"):
            regexes.append(re.compile(s[len("regex:"):]))
        else:
            exact.add(s)
    return exact, regexes


def main():
    argv = sys.argv[1:]
    src, names_path, out = argv[0], argv[1], argv[2]
    lines = open(src).read().split("\n")
    items, order = parse_items(lines)
    exact, regexes = load_names(names_path)
    # Names the shim defines by hand; never harvest, never follow.
    handwritten = {
        "Datum", "Oid", "TransactionId", "MultiXactId", "PgNode", "MemoryContextData",
        "MemoryContext", "NotBuiltinOid", "BuiltinOid", "FunctionCallInfoBaseData", "FmgrInfo",
    }
    want = set(n for n in exact if n in items)
    missing = [n for n in exact if n not in items]
    for name in items:
        if any(r.match(name) for r in regexes):
            want.add(name)
    # transitive closure over identifiers referenced BY VALUE; a type that is
    # only ever reached through a raw pointer stays opaque (layout not needed).
    PTR_RE = re.compile(r"\*(?:mut|const) ([A-Za-z_][A-Za-z0-9_]*)")
    opaque = set()
    queue = list(want)
    while queue:
        name = queue.pop()
        for text in items[name]:
            by_ptr = set(PTR_RE.findall(text))
            stripped = PTR_RE.sub("", text)
            by_value = set(IDENT_RE.findall(stripped))
            for ident in by_value:
                if ident in items and ident not in want and ident not in handwritten:
                    want.add(ident)
                    opaque.discard(ident)
                    queue.append(ident)
            for ident in by_ptr - by_value:
                if ident in items and ident not in want and ident not in handwritten:
                    opaque.add(ident)
    with open(out, "w") as f:
        f.write("// GENERATED by scripts/pgrx-harvest-bindings.py from the pgrx PG18 bindgen\n")
        f.write("// output (C-layout types and constants only). Do not hand-edit.\n")
        f.write("#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, clippy::all, unnecessary_transmutes, unknown_lints)]\n")
        f.write("use crate as pg_sys;\nuse crate::{Datum, MultiXactId, Oid, PgNode, TransactionId};\nuse crate::pgrust::mem::{MemoryContext, MemoryContextData};\nuse crate::pgrust::fmgr::{FmgrInfo, FunctionCallInfoBaseData};\n\n")
        emitted = set()
        for name, text in order:
            if name in want:
                f.write(text + "\n")
                emitted.add(name)
        f.write("\n// Opaque: reached only through raw pointers; the pgrust target never reads their fields.\n")
        for name in sorted(opaque):
            if name in emitted:
                continue
            # a type alias to a pointer (e.g. `pub type Relation = *mut RelationData`) is itself
            # by-value text; only emit opaque bodies for struct/union/enum items
            first = items[name][0]
            if re.search(r"^pub (?:struct|union|enum) ", first, re.M):
                f.write(f"#[repr(C)]\n#[derive(Debug, Copy, Clone)]\npub struct {name} {{ _opaque: [u8; 0] }}\n")
                emitted.add(name)
            elif re.search(r"^pub (?:type|const) ", first, re.M):
                f.write(first + "\n")
                emitted.add(name)
    sys.stderr.write(f"harvested {len(emitted)} items ({len(want)} requested incl. closure); missing: {missing}\n")
    if len(argv) > 3 and argv[3] == "--fns":
        fnames = [l.strip() for l in open(argv[4]) if l.strip() and not l.startswith("#")]
        sigs = parse_fn_sigs(lines)
        with open(argv[5], "w") as f:
            for fn in fnames:
                f.write(sigs.get(fn, f"// MISSING {fn}") + "\n")
        sys.stderr.write(f"dumped {sum(1 for fn in fnames if fn in sigs)}/{len(fnames)} fn signatures\n")


if __name__ == "__main__":
    main()
