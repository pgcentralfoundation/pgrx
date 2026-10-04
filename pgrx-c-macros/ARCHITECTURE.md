# C macro transpiler architecture

This document describes how to rebuild the `pgrx-c-macros` pipeline and its
integration with pgrx. It explains the data passed between phases, the algorithms
that establish C semantics, the generated Rust and C code, and the conditions
under which generation must refuse a macro. The implementation described here
is the current runtime transpiler. Its macro definitions do not promise const
evaluation or support for arbitrary C programs; independently proved object
constants use a separate binding path.

The intended readers understand Rust, C expressions, and basic compiler concepts.
An *identity* below means a C type or declaration, including distinctions that
Rust storage alone cannot express. A *place* means an addressable object, as
opposed to the value read from it. A *witness* means a compiler or Rust check
that establishes a fact needed by the next phase.

## 1. What the system produces

The input is a PostgreSQL installation, pgrx's version-specific wrapper header,
and that installation's C compilation settings. The output is:

- Rust `macro_rules!` definitions for supported function-like C macros and
  explicitly selected object expressions with zero-argument invocation.
- Independently compiler-proved integer object constants missing from bindgen.
- Shared Rust capability implementations derived from C declarations and the
  actual Rust bindings for the selected build.
- Generated C accessors, call adapters, static-inline wrappers and immutable
  declaration metadata, compiled into the active version's native archive.
- A report containing the selected profile, original definitions, provenance,
  analysis, emitted source, and reasons for skipping other macros.

Unselected object-like macros and external headers remain preprocessing context.
Ordinary CLI discovery still lists function-like definitions. Library callers can
select object roots; the binding build selects owned objects that depend on
declared variables, while pure integer objects are candidates for constants.
Original installed C definitions establish semantics. Test-only primitive names
survive deletion of handwritten Rust ports and confer no production signature.

```text
 pgrx-pg-config                 pgrx-pg-sys/include/pgNN.h
       |                                   |
       +------------+----------------------+
                    |
          installation + C arguments
                    |
          +---------v----------+
          | Compiler inspection|  Clang driver + libclang must agree
          +---------+----------+
                    |
           FrontendOutput
                    |
          +---------v----------+
          | Prepared session   |  function/object roots, constant + zero proofs
          +---------+----------+
                    |
          +---------v----------+       fresh bindgen output
          | Symbolic analysis  |                |
          +---------+----------+         BindingCatalog
                    |                           |
                    +-------------+-------------+
                                  |
                      +-----------v-----------+
                      | Capability planning   |
                      | ABI/storage validation|
                      | Rust macro lowering   |
                      +-----------+-----------+
                                  |
                    MacroGeneration + immutable C metadata
                 /                |                 \
        macro definitions    Rust support         C support + inline wrappers
                 |                |                 |
                 +--------+-------+           compile + archive
                          |                         |
                 formatted module tree              |
                          |                         |
                     pgrx-pg-sys <-------------------+
                          |
                   pgrx root exports

             trusted matching target build
                    |
 Complete target bundle: raw bindings + macros/report + wrapper/native archive
                    |
           bounded integrity/domain checks
                    |
           same publication/link path (no header inspection on import)
```

The transpiler owns the interpretation of the supported expression and statement
grammar. Clang owns preprocessing and supplies declaration and target facts.
The Rust compiler resolves the capabilities required by each invocation.
Clang does not type-check every possible macro invocation: the macro's formals
remain symbolic until the Rust caller supplies them.

## 2. Non-negotiable contracts

A reimplementation must preserve these contracts before adding support:

1. **One C environment.** Target facts, active macros, declarations, expansion,
   constant proofs, bindgen, and native compilation must agree on the selected
   inputs. A changed header or compiler environment invalidates previous facts.
2. **C identity survives storage.** C `long` and `long long` can both use Rust
   `i64` without becoming the same C type. Rank, nominal record/enum/prototype
   identity, pointer layers, and qualifiers remain explicit.
3. **Substitution preserves evaluation.** Each surviving argument occurrence,
   lazy branch, comma sequence, place computation, and unevaluated operand keeps
   its C behavior. Do not first bind every macro argument to a Rust local.
4. **Bindings are independent evidence.** Clang establishes C meaning; fresh
   bindgen output establishes available Rust paths and storage. Neither can
   silently repair the other's value or layout.
5. **Unsafe operations stay unsafe.** Generated code cannot turn an arbitrary
   pointer into a safe reference or make PostgreSQL calls safe on any thread.
   Caller obligations and PostgreSQL's error/panic boundary remain in force.
6. **Refusal is an output.** Missing proofs produce structured skips or a build
   error. A convenient spelling, a successful sample invocation, or matching
   widths does not substitute for a proof.
7. **Work is bounded.** Compiler children, source generation, parsing, expansion,
   and adapter generation have explicit limits. A hostile or unusually large
   macro cannot cause unbounded recursive processing.

C undefined behavior is outside the admitted invocation domain. The runtime
checks arithmetic conditions before an invalid Rust operation can occur, and
pointer/storage operations require the corresponding unsafe contract. This is
not a claim that all C programs become safe Rust. It also does not give C an
evaluation order it never specified: where C permits several operand orders,
the generated expression uses an allowed order.

## 3. Components and their ownership

| Component | Owns |
| --- | --- |
| `pgrx-c-macros` | Discovery, compiler agreement, expansion, parsing, analysis, reconciliation, demand planning, and source emission. |
| `pgrx-pg-config` | Installation/version resolution and PostgreSQL's recorded build settings. |
| `pgrx-bindgen` | The current bindgen invocation, collection of Rust binding facts, immutable C metadata bridges, complete target bundles, publication, Cargo invalidation, and native archive linkage. |
| `pgrx-pg-sys/src/c_macros/` | The reusable Rust implementation of modeled C value, conversion, place, and statement semantics. |
| `pgrx-pg-sys` generated code | Build-specific C identities, native storage bridges, field/call capabilities, and public macros. |
| `pgrx` | Reexports of the generated public macro modules. |

The CLI and the build integration share the library. The CLI is an observer and
standalone emitter; it does not collect pgrx's fresh bindgen catalog. Therefore
CLI emission with its empty catalog is not a prediction of everything a normal
`pgrx-pg-sys` build can emit.

## 4. Data model and phase boundaries

Keep the following objects separate even if another implementation uses
different names:

| Object | Required information | Why it is separate |
| --- | --- | --- |
| `MacroInventory` | Encountered definitions, owned tokens, token kinds, physical source spans, and diagnostics. | Discovery history can include redefined or undefined macros. |
| `MacroEnvironment` | Final active definition by name and its provenance resolution. | Expansion must use the final state, including restored macros. |
| `CompilationProfile` | Original main-file path, matched compiler identity, ordered arguments, target facts, overflow policy, unsupported options, and `BuildInputs`. | Every observation has a compilation context. |
| `DeclarationCatalog` | Named types, canonical structural shapes, records, fields, functions/prototypes, variables, enum constants, and verified builtins. | C identity and layout are independent of Rust storage. |
| `FrontendOutput` | The profile, environment, catalog, inventory, and initial dependency graph from one inspection. | Downstream consumers cannot assemble unrelated facts casually. |
| `ExpansionBatch` | Per-root expanded bodies or skips, original formal names, occurrences, dependencies, constant fallbacks, and consumed inputs. | Expansion observations belong to one requested batch. |
| `AnalysisSession` | A borrowed frontend plus expansion results, retained integer constants, zero-constant proofs, and the augmented graph. | Emission accepts a coherent session rather than a detached AST. |
| `Expression` | Arena nodes indexed by `NodeId`, root, token ranges, and optional structured statement body. | One syntax arena supports analysis and all emission contexts. |
| `MacroAnalysis` | Signature, formal/capture roles, uses, invocation/evaluation contracts, symbolic result types, dependencies, and candidate/skip status. | Candidate analysis is weaker than successful emission. |
| `BindingCatalog` | Actual crate-relative Rust paths, constant values, aliases, functions, record/enum/variable storage, bitfield metadata, and generated capabilities or rejection reasons. | Available Rust representations must be validated against C. |
| `MacroGeneration` | Per-root `MacroEmission` values and one `MacroSupportArtifact` containing Rust and optional C source. | Macros and their required support must be published together. |

The target model includes the C triple, byte order, byte and pointer widths,
function-pointer layout, `size_t` identity, integer widths/ranks, plain-char
signedness, floating representation and mode, supported C standard, basic character
encoding, and verified `offsetof` behavior. Structural types retain const and
volatile qualification at each relevant layer. Array bounds, incomplete arrays,
records, functions, and pointers remain structural relationships rather than
flattened strings.

All inventory/catalog data outlives the libclang translation unit that produced
it. Copy required observations into owned Rust data. Do not keep borrowed Clang
entities in a session or emitted report.

Collect structural types with a shared worklist and mark a canonical identity
visited before following its children; self-referential records must terminate.
`FunctionSignature.parameters = None` means an absent C prototype, not a
zero-argument function. Record member offsets are in bits; convert to bytes only
after establishing the access/layout contract. Attribute-sensitive typedefs
cannot become ordinary scalars just because canonicalization erased the attribute.

## 5. Resolving the installation and inspecting C

`PostgresConfig::resolve` uses pgrx's existing resolution rules. The first
positional version after a CLI subcommand accepts forms such as `pg18` and `18`.
Configuration includes `PGRX_HOME/config.toml` and the `PGRX_PG_CONFIG_PATH`
override. `pg_config` supplies the server include root and recorded C arguments.
The default input is the corresponding canonical pgrx wrapper, embedded in the
crate and materialized under PGRX_HOME independently of the source checkout.
Optional caller arguments follow the recorded settings; their order matters.

There are two different inspection paths:

- `scan` records preprocessing definitions through libclang and is sufficient
  for discovery output. The CLI `list` path uses recorded preprocessor/include
  settings, keeps definition history, and does not obtain the driver's final
  map or add the analysis path's CFLAGS.
- `inspect` establishes a compiler profile, final macro state, declaration
  catalog, target facts, inputs, and dependency graph for translation.

### Compiler agreement

Select a Clang executable compatible with the loaded libclang. Explicit hints,
the loaded library's location, candidate executables, and search paths participate
in selection. A usable executable alone is insufficient. Verify version family,
effective target, resource/header lookup, predefines, declaration observations,
and the driver's selected frontend configuration. Reject driver plugins or
configuration that would silently introduce another frontend.

Compare the driver's actual inclusion set with libclang's, and compare empty-input
predefined token maps without header-forced includes. Independently establish
fundamental typedef sizes, alignments, signedness, and expression result identities
through libclang and driver `_Static_assert` witnesses. Function-pointer layout
is a separate witness from object-pointer layout. Optional `offsetof` probes
must not hide a failed required fundamental witness. Protect proof operations
and fundamental fact macros against caller forgery.
The actual-inclusion comparison uses driver `-H` observations plus the main
file. `-M` availability dependencies are separate rebuild inputs. Remove
`-include` and `-imacros` from independent target probes so forced headers cannot
supply forged fundamental target facts. The current `-H` comparison can
conservatively reject a forced-include profile if the driver omits a header that
libclang reports; inspection fails rather than assuming those inclusion sets
agree. Supporting such profiles requires a complete independent inclusion witness.

Normalize arguments so the inspected language is C and required probe actions
can run without accepting options that write unrelated outputs or replace the
input language. Preserve language, target, include, macro, ABI, and semantic
settings. Classify known semantic flags explicitly. Unknown semantic settings
remain visible in `unsupported_options`; generation must not guess their effect.
Recorded MSVC fragments use Windows argument quoting rather than POSIX shell
escaping. Translate only reviewed options to GNU-style Clang semantics; refuse
unknown options instead of dropping a potentially relevant mode. This supports
a checked x86_64 Windows/MSVC LLP64 family, not arbitrary Windows compilers or
proof of backend execution on that platform.

Runtime options are ordered intermediate markers until the complete MSVC argv
has been assembled. `lower_msvc_runtime_flags` selects its final `/MD[d]` or
`/MT[d]` mode once across recorded fragments and explicit profile arguments.
Place the selected `_MT`, optional `_DLL` and optional `_DEBUG` predefines before
user `-D`/`-U` arguments, and retain the selected CRT plus `oldnames` as closed
cc1 dependent-library options. Static modes also retain the CL driver's standard
library visibility setting. These options preserve original native COFF linker
directives without requiring the newer `-fms-runtime-lib` driver switch.
Known option operands cannot become runtime selectors; refuse unknown arity when
folding is required. Bindgen's internally appended environment tail must not
contain a runtime selection that would make inspection and binding flags differ.
The inspected MSVC ABI can carry a numeric version suffix; use the same
`TargetFacts::uses_msvc_abi` decision for object extensions, archive conventions
and omission of the unsupported GNU PIC flag.

Signed arithmetic follows the recorded `Undefined`, `Wrapping`, or `Trapping`
policy. The current helper gate accepts the first two and refuses trapping
overflow. Reviewed LTO spellings are code-generation settings and do not change
the modeled C expression family. Native compilation later forces an ordinary
non-LTO object so the archive does not require Rust's LLVM to match Clang's LLVM.

### Declaration proofs beyond the first AST

Copy declaration facts into the owned catalog before releasing the translation
unit. Canonicalization can hide attributes, so follow typedef layers and reject
unmodeled arithmetic or ABI attributes rather than accepting their canonical
integer spelling. Record `packed` and `aligned` attributes can be represented
by the compiler's layout facts; other attributes still need a specific proof.

The initial parse may skip function bodies. Three bounded passes add facts that
cannot be inferred from a declaration's name or broad type category:

1. **Static inline definitions.** Gather internal-linkage static
   inline candidates, then reparse once with bodies enabled. Require an actual
   definition and agreement on linkage, inline status, result, parameters,
   variadic status, and calling convention. A declaration with braces nearby
   does not establish that the included definition is available. Retain physical
   source spans, original formals and bounded definition text from that same
   parse. Header contents are copied once per physical source, rather than once
   per function; the common input fingerprints identify bodies for native adapter
   hashes, which serialize only the prototype and linkage.
2. **Bitfield operations.** For each named bitfield whose containing record has
   a usable C spelling, probe unary-plus promotion, assignment-result type and
   promotion, and postfix-result type and
   promotion. Const fields acquire no write facts. Both compiler paths must
   accept the witness source. For volatile access, compare LLVM load/store units
   for the original field, an alignment-one `may_alias` record alias, and the
   containing-record path before admitting the unaligned volatile variant.
   Isolate individually rejected fields by bounded batch bisection; do not
   substitute a neighboring field's facts.
3. **Reviewed builtins.** Consider referenced `__builtin_bswap16`,
   `__builtin_bswap32`, `__builtin_bswap64`, and `__builtin_expect` only. Reject
   shadowed names and forged proof operations. A typed call establishes
   parameter/result identities; an independent bounded LLVM value/effect check
   establishes the byte swap or first-argument-preserving hint. Admit only
   reviewed instruction patterns, including the necessary local stack/debug
   forms. An unknown effect or width remains unavailable. These are direct-call
   capabilities, not first-class function addresses.

The builtin pass records per-name proof failures separately from inspection
errors. A changed environment, unreadable input, or failed required shared
witness must fail inspection rather than appear as an unsupported builtin.

### Discovery and the final active map

Collect physical definitions and their spans through the detailed preprocessing
record. Obtain the final active map from Clang's final macro dump, not by
replaying a stream of `#define` and `#undef` events. Push/pop macro pragmas can
restore definitions that event replay misses.

For each active definition, compare the function/object kind, parameter list,
and replacement tokens with discovered definitions. Establish a unique physical
origin or record unresolved/ambiguous provenance. Logical `#line` locations do
not establish physical file ownership. Keep the main file's original spelling:
canonicalizing it for invocation could change symlink-relative quoted includes.
Ignore comment tokens in matching and deduplicate repeated observations of the
same physical `(file, offset)` definition. Within one exact signature, repeated
definitions without a physical location describe one compiler or command-line
context; they do not create competing header origins. A source-less match remains
distinct from every physical match, so mixed origins still require an ambiguity
report. Tokenize the driver dump through a
protected replay with terminal markers so one literal trailing backslash cannot
splice the next dumped definition.

Public candidates are final active function-like definitions whose physical
header resolves inside `pg_config --includedir-server`. Canonicalize the ownership
comparison, so symlinks, `..`, and similarly named neighboring directories cannot
bypass it. `postgres_object_macro_names` applies the same ownership/provenance
rules to explicitly selected object roots. External definitions, unselected
objects, wrapper definitions outside the server tree, command-line macros, and
compiler predefines remain context.
If final provenance is unresolved or ambiguous but PostgreSQL-owned historical
definitions exist, retain the selected name for a provenance skip report rather
than silently losing it from the analysis results. `list` remains a historical
inventory; it can include a definition subsequently undefined or replaced.

### Snapshot and invalidation

`BuildInputs` records the working directory, present and absent file inputs,
SHA-256 content fingerprints, include/search directories, executable-search
directories, and relevant optional environment values. Dependencies come from
compiler dependency output and include/resource lookup observations. Decode
Clang's makefile filename quoting: ordinary Windows backslashes remain literal;
spaces, hashes, dollars, and line continuations have their specific encodings.

Verification must detect more than an edited included header. A new earlier
header in an include search directory, an executable in an earlier `PATH`
directory, a changed working directory, or changed preprocessing environment can
invalidate the inspection. Source-derived facts are checked around preparation
passes and again before the combined artifact is published.

These checks are not a filesystem transaction or an immutable copy of every
input. Callers must keep inputs stable through generation. A concurrent edit
and restoration entirely between checks can escape fingerprint detection.

```text
 inspect input snapshot S
           |
  verify files + environment
           |
  expand requested macros
           |
  verify S, even if expansion failed
           |
  prove retained constants
           |
  verify S, even if a probe failed
           |
  prove source zero constants
           |
  AnalysisSession for S
           |
  reconcile + lower + compile native support
           |
  verify S before publication

 A changed input at any boundary --> discard facts and inspect again
```

## 6. Preparing symbolic expansions

`AnalysisSession::prepare` expands the requested names as a batch. Its output
must preserve formal holes without assigning them one observed concrete type.
`prepare_objects` selects object-like expression roots, and
`prepare_with_objects` accepts separate object/function lists in one coherent
batch. Object expansion observes `NAME`, not a fictitious C `NAME()` invocation;
the generated Rust interface is `NAME!()`. Original object provenance and kind
remain in the report. Statement/declaration fragments do not acquire support
merely because they were selected as roots.

`prepare_with_inline_functions` accepts a separate list of compiler-owned inline
roots. Each must have a matching original internal-linkage static inline
definition, a complete fixed C prototype and the admitted default convention.
The original macro environment and expansion results remain unchanged. An active
C macro wins a name collision. A private immutable root map feeds protected
`(name)(arguments...)` call syntax to the existing analyzer, with private holes
outside the declared identifier namespace. Original formals and physical source
are retained separately for reporting and documentation. No preprocessing of a
synthetic definition or per-function translation unit is needed.

The normal native capability path preserves prototype conversions, qualifiers,
original C result identity and guard boundaries. Function operands evaluate once;
macro roots retain substitution, repetition and lazy/unevaluated contexts. Void
calls produce a `CExpression<()>`, whose `.get()` yields `()`. An explicit
`.is_true()` accepts C integer and bool predicate results without normalizing
their original C types. Unsupported prototypes, ABIs, transport or source budgets
retain structured refusals. Call source uses the shared root/token/source limits;
refusal records can cover the bounded 16,384-function compiler catalog.

### Symbolic probes

For each candidate, build an invocation using unique marker identifiers for its
formals. Establish that the marker namespace cannot collide with active macros,
declarations, or newly discovered identifiers. Markers are bare tokens, not
parenthesized expressions: adding parentheses could manufacture grouping absent
from the C replacement list.

Append instrumented probes to an overlay of the original main file. Invoke
Clang with the original main-file path and a virtual filesystem overlay pointing
to that instrumented content. This preserves quoted include lookup, symlinks,
main-file conditional logic, and context-sensitive file observations. Protect
the boundary against an original trailing line continuation and preserve source
line behavior. An arbitrary scratch file that merely includes the wrapper is
not equivalent.

Clang performs argument prescan, replacement substitution, nested rescanning,
macro suppression, and token formation. Extract each complete probe between
owned sentinels, recover formal occurrences, and tokenize its result. Require
complete and recognized output. Malformed probes must not swallow another
probe's boundaries; isolate compiler-rejected probes rather than treating a
partially successful batch as proof.

Inspect each root's active dependency closure before and during expansion.
Reject unsupported variadics, root stringification, dynamic preprocessing builtins
outside the invocation-location contract,
unproved pastes, ambiguous provenance, and budget overflows. A dependency's
origin span is useful for an explanation; it is not an exact source map for each
fully expanded token.

### Retaining integer symbols

Clang expansion would ordinarily replace object macros such as
`MaxAllocHugeSize` with their bodies. Preserve the source identifier when an
independent compiler witness proves that replacing its expansion with one
symbolic atom leaves the relevant expression shape intact. Both frontends must
agree on its integer type and value. Probes must run in the original environment
and must not interfere with unrelated definitions.

When atomic retention cannot be proved, keep the compiler-resolved expansion and
record `ConstantFallback { name, reason }`. Lowering emits an explanatory
`/* PGRX: ... */` comment. Its expanded integer value is not permission to invent
a Rust binding or overwrite a bindgen value. The C facts remain available for
later disagreement checks even when readable symbol retention fails.

The retention recipe is deliberately stricter than general expression support:

1. Collect referenced object macros from admitted closures, including synthesized
   references, and reproduce the original overlay environment.
2. Require a pure, concrete integer expression within the optional folding
   policy; calls, loads, mutations, uncertain overflow, and intermediate widths
   above 64 bits cannot establish this optional witness.
3. Driver-check constant-expression/type witnesses, evaluate with libclang,
   then independently driver-check the observed C type and numeric equality.
4. Retain literal metadata only when the whole grouped expression is one literal.
5. Require an atomic expansion: one literal/identifier or an outer balanced
   parenthesis pair enclosing the complete expression. Ungrouped `1 + 2` is
   not interchangeable with one named atom in every textual context.
6. Mask eligible macros with fresh markers after the original header, reexpand
   selected invocations, and restore the markers' original expansions in memory.
7. Require exact noncomment token equality with the authoritative ordinary
   expansion before replacing those markers with the original symbol names.
8. On optional witness failure, keep the original expansion and explanation;
   on changed inputs or real I/O failure, fail the session.

Numeric equality alone does not prove this transformation: masking can change
rescan, token pasting, or C precedence. Optional typed batches bisect failures
within a 32-attempt isolation bound.

### Integer bindings omitted by bindgen

`probe_integer_object_constants` is separate from atomic name retention. The
binding build selects missing owned numeric object candidates, then proves valid
constant initialization, canonical integer identity, and value with libclang and
independent driver witnesses. Closed literal-suffix pastes require the compiler's
actual typed result. Runtime calls/loads, dynamic builtins, incomplete facts, and failed
witnesses cannot supply constants. Budgeted diagnostic isolation refuses bad
candidates without treating a successful sibling as evidence for them.

Rust storage follows the proved target signedness/width and `size_t` identity.
Only absent bindings are supplemented; existing value disagreements still skip
macros and their dependents. `ObjectIntegerConstants` retains accepted facts and
rejected reasons in the report. Optional expression-name retention can fail even
when a missing constant is proved: numeric equality does not authorize changing
an ungrouped replacement list into one expression atom.

### Closed token pastes

Some `##` expressions are closed over fixed tokens and can be resolved during
generation. Open pastes involving caller spelling still cannot be implemented
by pretending Rust expression fragments expose their original C tokens.

The closed-paste proof combines ordinary expansion with independent diagnostic
probes. Poison possible synthesized active identifiers so Clang exposes their
spelling origin in scratch-space diagnostics. Accept only diagnostics owned by
the exact probe, with one complete identifier, a valid source/caret excerpt,
known scratch origin, and the completion witness. Support both numbered and
legacy plain Clang excerpts without accepting mismatched layouts.

Before the plain-marker poison pass, run braced symbolic operands and traps for
reviewed preprocessor builtins. A caller-dependent paste must fail even when a
later expansion would erase the evidence. Define probe wrappers before poisoning
names: Clang exempts ordinary old definition tokens but checks newly pasted
identifiers before rescanning them. Poison active macro/dynamic/probe names and
require a unique final `#error` completion marker. Pasted builtins or fabricated
probe names are refusals. This proof currently audits Clang 6 through 21's
builtin families; an unaudited major refuses closed-paste support.

Iterate discovered synthesized dependencies within the finite compiler-pass
budget. Add them to the dependency graph. Reject erased operands, dynamic helper
names, probe collisions, incomplete diagnostics, or unknown error shapes. The
proof of an observed final paste is necessary because the ordinary lexical graph
cannot see an identifier assembled from several tokens.

### Source zero identity

C null-pointer constants include integer constant expressions with value zero.
An ordinary stored integer containing zero is not the same thing. After
expansion, identify eligible pure integer subexpressions and use compiler
witnesses to prove their integer-constant-expression status and zero value.
Select maximal pure integer subtrees bottom-up to avoid copying overlapping
source quadratically. Require enum-initializer integer constant expressions
under `-Werror=integer-overflow`; compare libclang enum results with independent
driver LLVM constant globals. Only their agreement grants zero metadata.
Record the proved `NodeId` values in the session. Do not assign null-constant
identity to an arbitrary native argument, loaded variable, or integer result.

## 7. Parsing the supported C grammar

The parser consumes the complete expanded replacement. It is a bounded parser
for the forms the emitter understands, rather than a second full C compiler.
Filter comments while retaining original token indices for diagnostics.
Construct an arena of `ExpressionNode { kind, tokens }`, where child references
are indices into the same arena.

### Expression grammar

Use precedence parsing for ordinary expressions. Preserve explicit group nodes,
even if structural comparisons later ignore groups. From tighter binding to
looser binding, the modeled operators are:

| Level | Forms |
| --- | --- |
| Postfix | Calls, indexing, `.`/`->`, postfix `++`/`--`. |
| Unary/cast | Prefix updates, unary `+ - ! ~`, dereference, address, casts, `sizeof`, alignment. |
| Multiplicative | `* / %`. |
| Additive | `+ -`. |
| Shift | `<< >>`. |
| Relational | `< <= > >=`. |
| Equality | `== !=`. |
| Bitwise | `&`, then `^`, then `|`. |
| Logical | `&&`, then `||`. |
| Conditional | `condition ? then : else`. |
| Assignment | `=` and supported compound assignments, associated to the right. |
| Comma | Sequence left effects before the right result. |

Represent parameters, identifiers, literals, groups, unary/binary operations,
casts, conditionals, commas, calls, members, indexing, dereference, address,
assignment, and updates as distinct node kinds. `sizeof`, alignment, type-formal
casts, and `offsetof` have dedicated structural nodes. A member-name formal is
an identifier role; a type formal is a type role, not a value expression.

Concrete type spellings must resolve through the C catalog. For a formal that
could be either a cast type or a parenthesized value/callee, collect independent
evidence for its type role and reparse. Do not decide that `(T)(x)` is a cast
merely because choosing that interpretation makes generation possible.
Bound pointer/qualifier declarators rather than accepting arbitrary textual
types. `offsetof` keeps a record designator and ordered field path; offsets do
not require loading those fields. Reparse once after gathering type-hole evidence
across the complete arena. Bare GNU alignment of `T` alone does not prove that
`T` is a type; pointer/qualified forms, established sizeof-type and offset forms
provide stronger evidence.

Argument lists parse assignment expressions with top-level commas as separators;
a grouped comma is an expression. The conditional's middle operand can contain
comma. Grouping analysis distinguishes `f(x)` from `f(x * 2)`: the latter does
not protect the textual substitution of `x` from other operators.

Integer literals retain original spelling, radix, magnitude, and `U`/`L`/`LL`
suffix metadata. Decimal and nondecimal literals have different candidate C
type lists. Select the first representable C type under the inspected target
rules. Unary minus is a separate operation, not part of literal magnitude.
Basic character constants require the established execution-character-set
contract. Ordinary closed strings decode the verified ASCII subset and modeled
escapes, concatenate adjacent literals, preserve embedded zero bytes, and append
one final zero. They lower to immutable static C char-array places with exact
extent; value context decays the array, while size/address contexts retain it.
They confer no write permission even though C string-literal syntax has array
type. Wide/UTF-prefixed strings, unproved encodings, out-of-range escapes and
unrepresentable integer magnitudes produce explicit errors.

Clang can resolve closed dependency stringification into such literals. A
dependency string containing formal markers retains its C text template and
holes, but is admitted only as a grouped direct `const char *` argument of a
fixed nonvariadic native void diagnostic. The emitted holes stringify outer
Rust invocation tokens without evaluation; ASCII token spelling is checked.
This contract does not promise a caller-dependent C array extent. Returning,
addressing, measuring, concatenating, or doing pointer arithmetic on these
diagnostic holes is refused, as is root `#` stringification.

Invocation `__FILE__` and `__LINE__` remain symbolic until Rust expansion and
use `file!()`/`line!()` rather than generator locations. Filename bytes become
static zero-terminated storage; the line must fit the inspected C int. A
dependency stringification of these markers remains a refusal because it would
mix C preprocessing spelling with a Rust source-location contract. Other dynamic
builtins, such as `__COUNTER__` and `__func__`, stay unsupported.

### Statement grammar

The statement parser shares the expression arena. It understands expression
statements, local declarations, blocks, `if`/`else`, returns, and the supported
`do { ... } while (0)` wrapper. Record source order, lexical scopes, first-return
location, whether every path returns, and whether an explicit outer boundary is
needed to avoid C's dangling-`else` behavior.

Reject unsupported loops, jumps, declaration shapes, and initialization forms.
Do not permit locals to shadow formals or other introduced locals, change an
earlier identifier's binding, or escape their source scope. Definite-initialization
analysis rejects local reads that are not established as initialized on the
required paths. Reads are checked before granting initialization from an
initializer or a full-expression direct assignment. Taking or passing an address
proves no initialization; compound assignment reads the previous value. At joins,
intersect continuing paths and exclude returning paths. The parser must consume
the entire replacement; a supported
prefix followed by unknown tokens never becomes an accepted macro.

## 8. Symbolic semantic analysis

`session.analyze(name)` analyzes the trusted expansion and restores original C
formal names after interpreting its markers. The analysis is an invocation
family, not an inferred concrete function signature.

Build a `types` vector parallel to the expression arena. A `TypeExpression` is
one of:

- A concrete fundamental integer identity.
- A concrete external C type from a declaration.
- A formal parameter's unknown type.
- A promotion of another node's type.
- The common arithmetic type of two nodes.
- A deferred capability-constrained type.
- Void or an admitted direct compiler builtin.

The vector references node indices for promotion/common-type relationships.
Resolve named constants independently and associate their facts with source
nodes. Do not resolve a generic operand by probing one convenient `u32` call.

Walk uses to establish each formal's syntactic roles: value, type, identifier,
field designator, unused, or unknown. Readable/writable place requirements are
contextual semantic capabilities established later, not a parameter role.
Track occurrence positions and evaluation obligations.
An undeclared caller-scope identifier can become an additional explicit capture
argument after the original formals. Declared C variables/functions/constants
use the catalog instead. Captures stay separate from formals even if a nested
expansion introduces the same spelling as an unused outer formal.

Determine an invocation contract from grouping and statement shape:

| Contract | Caller requirement |
| --- | --- |
| Parenthesized scalar expressions | Ordinary expression substitution has an established grouping boundary. |
| Atomic arguments | An ungrouped parameter accepts one atomic token tree or an explicitly parenthesized argument. |
| Explicit expression boundary | Caller requests the meaning of the parenthesized C invocation with `@__pgrx_c_expression`. |
| Statements / returning statements | Preserve ordered statements, scope, captures, and any enclosing-function return. |
| Explicit statement boundary | Caller requests the braced C invocation with `@__pgrx_c_statement`. |

Analysis records lazy branches, repeated occurrences, unspecified operand order,
the exclusion of undefined operations, and overflow policy. Supported output
currently reports `RuntimeOnly`. A macro made entirely of integer syntax still
does not become a Rust const expression merely because its constructor is const.

## 9. The Rust C-semantics runtime

The runtime lives in `pgrx-pg-sys/src/c_macros/support.rs` and its expression,
result, record, enum, literal/null, and statement support files. It is exposed
as the hidden `pgrx-pg-sys::__pgrx_c_macros` module so exported macros can use
`$crate` paths without requiring imports in the caller.

### Integer identity and conversions

`CInteger` markers encode representation, width, signedness, rank, promoted
identity, and value-boundary identity. `CValue<K>` stores a value tagged with
marker `K`. Compiler facts select LP64, LLP64, or ILP32 representation;
plain `char` can be signed or unsigned, independently of Rust's native `c_char`.
The baseline families are:

| C family | Bits | Rank | Ordinary promotion |
| --- | ---: | ---: | --- |
| `_Bool` | 8 | 0 | `int` |
| `char`, signed/unsigned `char` | 8 | 1 | `int` |
| signed/unsigned `short` | 16 | 2 | `int` |
| signed/unsigned `int` | 32 | 3 | Same kind |
| signed/unsigned `long` | 32 or 64 | 4 | Same kind |
| signed/unsigned `long long` | 64 | 5 | Same kind |
| signed/unsigned `__int128`, when available | 128 | 6 | Same kind |

The frontend independently proves each fact; a triple alone cannot supply the
table. Generated `__pgrx_c_types` aliases associate inspected integer typedefs
with their precise C marker. Use those identities for `int64`, `Size`, timestamps
and pointer-sized operands instead of selecting C long from storage width.
Missing or incompatible target facts fail the support gate.

`CSignedChar` and `CUnsignedChar` remain separate nominal types backed by `i8` and
`u8`. Primitive operands and pointers retain those identities even when
`core::ffi::c_char` aliases the same Rust primitive. Plain-char operands require
an explicit `CValue<CChar>` or `Pointer<CChar, _>` tag; native fields, function
signatures, and generated string arrays receive their compiler-established tag.

Implement integer promotions first, then usual arithmetic conversions:

1. Equal promoted types keep that type.
2. Equal signedness selects the greater rank.
3. Mixed signedness selects the unsigned type if its rank is at least the
   signed type's rank.
4. Otherwise select the signed type if it represents every value of the
   unsigned type.
5. Otherwise select the unsigned counterpart of the signed type.

The compiler separately measures the preferred scalar and array alignments used
by C type operators. A singleton-record witness supplies ABI storage alignment;
these can differ, as with 32-bit x86 `double` and `long long`. Generated
`__pgrx_c_alignment` constants drive the runtime's alignment operators under
`pgrx_c_alignment`, while loads and layout checks keep using actual binding
storage. Nested arrays and qualifier/storage adapters preserve the appropriate
C alignment. An unavailable extension type has a zero sentinel and is rejected
at monomorphization instead of acquiring a guessed C representation.

`Common<Rhs>` encodes that finite table. Arithmetic and comparison helpers
convert both operands to the resulting identity. Shifts promote operands
individually and retain the left promoted result type. Cast helpers implement
C conversions, including boolean truth conversion before narrowing. Logical
operations and comparisons produce C `int` results, not Rust `bool` values.

Accept native Rust inputs only where their C identity is unambiguous. An `i64`
does not identify whether the caller means C `long` or `long long`; use an
explicit tag. The same caution applies to pointer-sized integers. Generated
bridges establish pgrx's opaque `Oid`, `TransactionId`, and `Datum` C identities
from the selected headers rather than guessing from their Rust wrappers.

Unsigned arithmetic wraps. Signed arithmetic uses the inspected overflow
policy. Under the undefined policy, reject overflowing admitted operations;
under wrapping, wrap their representation. Division/remainder by zero,
signed minimum divided by minus one, invalid shift counts, and signed left-shift
domain restrictions need their own checks. Signed right shift uses the agreed
Clang arithmetic-shift choice.

### Values, types, places, and results

`CType` connects a semantic marker to native storage and the value obtained by
reading that storage. `NativeType` maps a validated Rust representation back to
a C marker. `CExprValue` and `IntoExpression` distinguish expression values from
storage. These traits are sealed or use crate-owned generated registrations;
outside code cannot add arbitrary implementations that bypass the proofs.

Important value families include tagged integers, floats, qualified typed
pointers, arrays, nominal functions, records, opaque records, enums, and void.
`Place<M, Q>` describes an addressable object with marker `M` and qualifier `Q`.
It carries access metadata such as volatility and unaligned access. Constructing
a place does not make a load or store safe. `ReadPlace`, `WritePlace`, field,
offset, and qualifier capabilities govern those later operations.

Keep explicit casts separate from implicit assignment/prototype conversions.
Implicit assignment/prototype conversions cannot discard const or volatile
qualification. Explicit C pointer casts can change qualifiers; they still do not
establish the safety of a later access. Object/function
pointer identities remain distinct; nested pointer compatibility requires the
correct structural proof. Pointer arithmetic, subtraction, comparison, loads,
and stores retain their C allocation, alignment, provenance, initialization,
and aliasing requirements.

`CExpression<V>` wraps an already evaluated public result. `.get()` extracts
native storage once; `.into_value()` keeps its tagged C value. A macro returning
an integer is therefore not generally used as a bare native Rust integer until
the caller extracts or converts it. Composition through generated macro syntax
can retain additional source metadata; an evaluated Rust value boundary must
drop source-only null-constant or bitfield information where C would lose it.

Raw aggregate values use `RawRecordValue<R>` and `MaybeUninit<R>`. Reading one
initialized field must not materialize invalid bool/enum values or read
uninitialized neighboring fields or padding. `.get()` for that family returns
`MaybeUninit<R>`.
`assume_initialized` is unsafe and requires proof for the whole Rust record.
Enum expressions retain nominal C identity and compatible integer storage;
unnamed C enum values cannot justify constructing an invalid Rust enum.

### Floats and compiler builtins

Native binary32/binary64 values and conversions need independent target facts.
Composed floating arithmetic has stricter requirements for contraction,
evaluation precision, and recorded floating modes. A changed runtime rounding
or exception environment and overriding source pragmas are outside that
contract. Floating literals are not implied to be supported by support for
native float values.

Verified byte-swap builtins lower to Rust operations with the C result identity.
Branch hints lower to Rust support with cold paths while preserving C operand
evaluation and result identity. Compute fixed-value and projected-expectation
facts bottom-up, then classify hints as `Static`, `Dynamic`, or `Overridden`.
Walk back through grouping and casts so an outer expectation suppresses advice
from an inner projected expectation; a dynamic outer expected value also
suppresses an inner static hint. Both original operands and conversions still
execute. Unknown builtins and context-dependent preprocessing constructs
remain refusals.

## 10. Contextual Rust macro emission

One C expression has different translations depending on its use. For example,
an argument used in `sizeof` must not execute, and an argument used as an
assignment destination must retain its address. The emitter therefore supports
value, writable-place, readable-place, size, and discard contexts.

```text
 caller supplies tokens
          |
 per-macro argument normalizer
          |
 bounded shared token classifier
          |
 descriptors: native / literal / place / generated call / unused
          |
 enclosing C operator requests a context
          |
          +-----------+-----------+-----------+-----------+
          |           |           |           |           |
        value       place     read-place     size       discard
          |           |           |           |           |
      evaluate      write        read        type       effects
     and convert   address      address     witness      only
```

Preserve original ASCII C formal names as Rust metavariables, such as `$privs`.
Rust keywords can name metavariables, but `$crate` is reserved. Unsupported
formal spellings produce a skip rather than silently replacing every name with
an index. Synthetic capture and return-marker names are collision-checked.
Type-only parameters use `ty` fragments; member designators use `ident`;
contextual value operands use token descriptors.

For a normal value argument, classify supported token forms without evaluating
them. Recognize calls to macros known to be generated, native expressions,
literal forms, place forms, and unused arguments within a bounded token grammar.
An unrelated Rust macro can be supplied as an explicit native expression using
`(@__pgrx_c_native [some_rust_macro!(argument)])`. The classifier is not a C
parser for arbitrary Rust expressions.

Do not first capture an operand as an opaque `expr` fragment when later macro
stages need to recognize a generated invocation and request its place or size
context. That would erase precisely the structure needed to compose C macros.
Generated helper names use `$crate` paths; user tokens retain caller context.
Expression macros expose value, place, read-place, size, and discard entry points.
An unsupported context contains `compile_error!` rather than removing otherwise
useful value behavior. Zero-argument macros omit argument normalization, but can
still have these internal context arms.

### Operator lowering rules

| C operation | Lowering rule |
| --- | --- |
| Integer/scalar arithmetic | Runtime operation trait with C promotions/common type and selected overflow marker. |
| `&&`, `||` | Rust lazy control flow plus C truth conversion and C `int` result. |
| `?:` | Evaluate the condition and exactly one arm; determine the common C result without executing the other arm. |
| Comma | Discard the left result after its effects, then evaluate the right result. |
| Assignment | Compute the destination place once, apply C implicit destination conversion, store, return the C assignment result. |
| Compound assignment / update | Compute the place once, preserve the read-modify-write behavior and prefix/postfix result distinction. |
| Address/member/index | Retain raw places, declared qualifiers, promoted anonymous-field paths, and array behavior. |
| `sizeof` expression | Use a type-checkable unexecuted witness; retain arrays rather than first decaying them to pointers. |
| `sizeof`/alignment type | Resolve a verified concrete or formal type marker; return C `size_t`. |
| `offsetof` | Use separately proved offset-path metadata; no load or place projection is required. |
| Function call | Apply C prototype conversions and validated callable ABI; preserve FFI guard and result validity. |
| Return statement | Return from the enclosing Rust caller after C assignment conversion to its established return marker. |

Unevaluated witnesses can still require Rust declarations and capabilities so
the generated expression type-checks. Their code does not run. Array length,
bitfield restrictions, incomplete objects, and pointer compatibility must not
be inferred by evaluating an otherwise unevaluated operand.

### Statements and caller returns

Render source-order statements directly into the macro expansion. Never put a
C `return` inside a closure that would return from the closure instead of the
caller. Use `return_value` where the enclosing Rust return type has an
unambiguous native C identity, or the generated explicit `@__pgrx_c_return_as`
entry with a supplied C marker where it does not.

Initialized locals receive their declared C storage and assignment conversion.
Track definite initialization across branches and scopes. Generated statement
guards inspect argument token spellings against introduced local names: Rust
macro hygiene cannot silently change C's textual local-name capture. Reject
uncertain capture shapes. Preserve `if` branch laziness and explicit statement
boundaries.

Some expression-like return helpers can be used with a caller-written Rust
`return`; a macro whose original C body contains `return` instead emits that
return itself. These are different analysis contracts, not one blanket rule
for names containing `RETURN`.

## 11. Readable symbols, casts, and cross-macro calls

Literal output retains radix and recognizable source spelling where possible.
For example, an original `0xFFFFFFFF` stays hexadecimal with an appropriate Rust
storage suffix, rather than being rendered as decimal. Its C type still comes
from the literal rules, not from the human-preferred spelling.

When a named constant has an actual Rust binding, compare its numeric bindgen
value with the independent C witness before emitting `$crate`'s symbol path.
Retain the C semantic marker even if bindgen chose a different Rust integer
storage type. Binding-rewritten values such as opaque IDs use their validated
representation bridges. If bindgen and Clang disagree, record both values
and skip the macro; emit the build warning when `PGRX_MACRO_DEBUG=1`. Do not
correct the binding behind the user's back.

Preserve cast aliases for every verified C/Rust alias, not for a hand-maintained
set of names. A cast spelled `AclMode` can show the Rust alias `AclMode` while
also carrying its C rank marker. Rust alias spelling alone cannot distinguish
`unsigned long` from `unsigned long long` if both are `u64`.

For cross-macro calls, start from the trustworthy fully expanded trees, then
recover readable calls only when equivalence is proved:

1. Recognize an original whole replacement that is a direct function-macro call.
2. Independently analyze the callee's prepared expansion.
3. Check arity, nonrecursive dependency conditions, and compatible body shape.
4. Match the callee tree against the caller tree; each callee formal is a hole.
5. Require repeated holes to match identical caller subtrees and concrete C facts.
6. Recover every argument; an unused callee formal whose argument is lost cannot
   be guessed.
7. Require that the callee is actually available in the emitted macro set.
8. Emit the nested call with deferred contextual operands so argument evaluation
   still follows substitution in the callee.

A recovered bare/grouped formal forwards its existing argument descriptor.
For a composite recovered argument, construct
`(@compiled [value] [place] [read_place] [size])`: each bracket contains Rust
*token source*, not a value computed during normalization. The callee selects
only the context demanded by its enclosing C operator. Unsupported alternatives
contain a context-local `compile_error!`; discard evaluates the value alternative
for effects and ignores its result. This preserves repeated substitutions,
places, and unevaluated operands without creating eager temporaries.

```text
 BUFFERALIGN(LEN) --> TYPEALIGN(ALIGNOF_BUFFER, (LEN))
          |                         |
   expand + analyze          expand + analyze
          |                         |
          +---- compare trees ------+
                        |
              equivalent + available?
                  /             \
                yes              no
                 |                |
          TYPEALIGN! call     expanded Rust body
                             + PGRX explanation
```

Statement delegation is more restricted: expression-tree equality does not
prove that discarded effects, local declarations, and scopes are equal. Only
the admitted simple whole-body wrapper cases use the current proof. Expectation
hints have additional equivalence rules so a preserved call cannot erase or
retain a different fixed branch hint.

## 12. Reconciling bindings and selecting support

The build integration parses the Rust that its current bindgen invocation just
produced. It must never read checked-in `src/include/pgNN.rs` as target facts;
those snapshots serve documentation builds and can describe another platform.

`RustBindingType` represents actual unit, bool, fixed/pointer-sized integers,
floats, pointers, arrays, functions, nullable `Option`, `ManuallyDrop`,
`MaybeUninit`, incomplete-array helpers, and named paths. Resolve aliases and
record/enum paths structurally. Keep renamed fields, anonymous record edges,
function `link_name`, ABI, prototype, guarded status, and cshim linkage facts.
Unknown, conflicting, or ambiguous storage witnesses must remain failures.

### Demand planning

`demands::plan` walks the analyzed roots and builds requests for fields, offsets,
functions, callable bodies, original addresses, types, callbacks, and enums.
It uses a finite worklist to propagate possible value/type categories through
the arena. Operators, assignment destinations, member paths, known prototypes,
and declared results narrow the possible families. Index records and promoted
members once, group prototypes by arity, and cache compatibility queries.

An unknown operand keeps its family open. It does not justify selecting one
observed record or omitting valid generic callers. For example, a declared
member result can restrict the owner of a following field access, while an
unconstrained caller-supplied record may retain several compatible owners.
Unevaluated calls retain capabilities needed for Rust type checking; a function
address alone does not retain a callable wrapper.

Generate adapters in dependency order:

```text
 initial requested roots
          |
     demand plan
          |
 preliminary callback identities
          |
 function/native call adapters
          |
 final callback capabilities
          |
 required type closure
          |
 enum + field + offset + original-address adapters
          |
 register capabilities in a derived BindingCatalog
          |
 emit batch + propagate required failures
          |
 plan demands again from successful roots
          |
 if requests changed: regenerate adapters once
          |
 publish successful macros + retained support
```

This is one final demand recomputation, not an arbitrary fixed-point loop.
Marker identities come from stable C/declaration/profile facts, so pruning
support cannot renumber the identities already referenced by emitted macros.

All relevant ABI and storage witnesses participate in validation before pruning.
A rejected or unselected alias still matters when deciding whether one Rust
function-pointer storage type uniquely identifies a C prototype. Omitting that
witness could create an unsound generic input bridge.

### Records, enums, and callbacks

Complete record registrations share sealing, input conversions, and marker
implementation. Actual Rust `Copy` bounds permit ordinary native record inputs.
A complete non-`Copy` record can still support raw pointers, individual field
operations, and validity-aware `MaybeUninit` transport when its ABI is proved;
it does not thereby acquire a conversion from an arbitrary initialized Rust
value. Incomplete records retain an opaque identity and cannot acquire
complete-object size or value operations.

Enums retain nominal C identity separately from compatible integer kind and
Rust storage. Check each native Rust enum discriminant before enabling its
encoder. Primitive storage and pointer compatibility use shared tables without
merging two enum identities. Layout checks can be deduplicated after every
identity has been validated.

Callbacks retain exact C prototypes and actual Rust storage. Supported native
storage is a nullable `Option<unsafe extern "C" fn(...) -> ...>` or corresponding
`"C-unwind"` function pointer, with a fixed nonvariadic C calling convention,
exact arity, and validated argument/result representations. Install
provisional identity/storage markers so nested callback prototypes can be
reconciled, then validate candidates and propagate failed nested dependencies
through a reverse queue before emitting final call capabilities. Provisional
markers are internal reconciliation evidence, not permission to call an
unvalidated prototype.

Physical function-pointer implementations share selected ABI/arity families,
without a fixed arity ceiling, but nominal C identities do not merge merely
because Rust sees equal function types. Generate an implicit native callback
input only when the complete catalog proves one validated identity for that
storage; otherwise callers use tagged `FunctionValue` values. Rejected,
unrequested, and unresolved catalog witnesses all participate in this check:
not seeing an incompatible witness among selected macros does not prove
uniqueness. Identity-only use does not retain nested callable machinery
unnecessarily.

Selected validated typedefs also receive aliases under `__pgrx_c_callbacks`,
mirroring their actual Rust binding module paths. Each alias names
`FunctionValue<its nominal marker>`; `.new(pointer)` accepts only that marker's
exact native storage. This explicit selection adds no ABI cast and does not
pretend ambiguous raw storage globally identifies one C prototype. Conflicting
paths fail; rejected/unproven prototypes receive no constructor. The safe
constructor does not call the pointer or discharge the unsafe invocation contract.

## 13. Generated C and native safety

The macro bodies are translated to Rust. Some required primitives use generated
C because Rust bindings alone do not provide the original C access or function
identity. The production C artifact includes these families:

| Family | Generated C operation | Why C is needed |
| --- | --- | --- |
| Bitfields | Read/write the original named bitfield, with qualified/alignment variants. | Bindgen's backing storage/accessors cannot safely read an arbitrary partly initialized record as a Rust value. |
| Native function adapters | Call the original declared function, including static inline definitions, with validated ABI transport. | A static inline function can have no ordinary exported symbol; aggregate and enum transport need validity-aware bridges. |
| Original-address getters | Return the original C function address. | A Rust error-guard wrapper or a generated call thunk has a different address. |
| Bindgen static-inline wrappers | Bind the original inline definition with ordinary guarded Rust FFI. | PostgreSQL versions can replace macros with inline functions; availability must not depend on `cshim`. |
| Immutable declaration metadata | Initialize module magic and declare V1 function info through original C macros. | Initializer/declaration fragments are not runtime expression macro bodies. |

### Immutable declaration data

The binding integration initializes a static const `Pg_magic_struct` with the
original `PG_MODULE_MAGIC_DATA`, using its actual object/function invocation
kind, and invokes `PG_FUNCTION_INFO_V1` with a profile-specific native name. Its
actual emitted getter supplies the immutable function-info record. No copied
magic version arithmetic, API-version literal or aggregate initializer supplies
these values. Exported Rust entry points and custom static module name/version
controls remain in pgrx; module magic is cached once.

Before Rust copies or borrows these records, recursively reconcile exact binding
paths, field types, fixed arrays and nested records against the Clang catalog.
Emit C and Rust size/alignment and field-offset assertions, plus typed Rust field
witnesses. Unrestricted integer storage and raw pointers with fully proved scalar
pointee types/qualifications must be established;
bools, enums, references, function pointers, packed/unresolved storage and other
unproved aggregate validity do not pass merely because their bytes fit. These
getters read immutable linked-module static data only: no backend state, callback
or PostgreSQL ERROR can occur, so no backend guard is introduced around them.
Profile-specific link names and hidden helper visibility prevent ordinary helper
interposition from substituting another extension's layouts.

Runtime expression macros still lower to Rust; their C helpers provide validated
access/call primitives. Declaration metadata deliberately uses the original C
initializer/declaration instead of copying it into Rust. Branch-prediction hints
and byte swaps use Rust support. Catalog facts and retained demands generate
runtime primitives without a macro-name signature list or handwritten port.

### Layout and access proofs

Reconcile native record kind, size, alignment, direct and promoted member paths,
field offsets, qualification, and actual Rust storage. Use eager Rust layout
constants and C `_Static_assert` checks where applicable. An uncalled typed
Rust projection witness still forces the compiler to check exact native storage
and promoted paths. Share projection implementations without dropping each
field's individual proof.

Ordinary field access uses raw pointers and aligned/unaligned or volatile
operations selected from validated access facts. Never construct a reference
to a packed field to get its address. Offset-only capabilities are independent
from projection and load capabilities. Flexible array and anonymous-member
relationships require structural validation, not name-based guesses.

Generate a bitfield capability with this sequence:

1. Recover the fresh bindgen accessor's storage-unit path, bit offset, and width,
   including the backing unit's bounds. Treat its `.get(offset, width)` shape as
   layout evidence; never invoke that getter to read arbitrary partly
   initialized record storage.
2. Reconcile the unit and field with Clang's bit offset, width, declared type,
   promotion type, assignment result, and postfix result. Reject missing facts,
   zero-width fields, and anonymous paths whose access has not been proved.
3. Generate C getters/setters that name the original field. C performs width
   narrowing and preserves adjacent bits. Use static layout assertions to
   validate any alignment-one `may_alias` alias.
4. Emit aligned, volatile, unaligned, and unaligned-volatile variants only when
   the relevant compiler access facts justify each variant. In particular,
   unaligned volatile access needs the independently matched access-unit proof.
5. Expose only the corresponding read/write place capabilities in Rust. Keep
   width and promotion metadata in `CBitfield<Base, Promoted>` until a C value
   boundary removes it; assignment and postfix results can have different
   semantic types.

A successful ordinary getter is not proof of a qualified getter, and matching
record layout is not proof that a whole backing byte unit can be loaded as an
initialized Rust value.

### Function ABI and PostgreSQL errors

Validate calling convention, complete prototypes, each parameter/result
representation, pointer layers, arrays, records, enums, and callback storage.
Reject unsupported variadics and ambiguous representations. ABI transport may
use by-value `MaybeUninit<BindingRecord>` for records with checked C/Rust layout,
and the compiler-compatible integer for enums. Decode after the call into raw
record values or numeric `CEnum` values, preserving nominal C identity. Convert
an enum value to an actual Rust enum binding only through a discriminant-checked
encoder, which rejects values with no Rust variant. Do not read padding or
uninitialized fields as initialized bytes, or construct a Rust enum with an
unnamed discriminant. Materializing a Rust record requires valid initialized
fields and its ownership contract; padding need not be initialized.
Plain-char field storage can have a different Rust sign while preserving every
byte, but that does not prove a callable ABI: narrow argument/result extension
attributes also matter. Direct C thunks therefore transport the compiler's
canonical char scalar. Indirect calls reinterpret only function-pointer storage
and invoke the exact original C signature, including its char sign; callers must
supply an address that actually has that prototype. A differently declared Rust
callback does not satisfy that obligation merely because its bytes have the same
width. Callback representation adaptation requires its own proved unchanged
ABI/layout before transmuting the exact function-pointer representation.

For adapters that can call PostgreSQL, `BindingCatalog::ffi_boundary` supplies
the crate's existing `pg_guard_ffi_boundary` path. Perform fallible argument
conversions, callback-null checks, and representation adaptation before the guard;
decode the result after it. Assert that generic arguments and captured native
storage do not need `Drop`. The guard closure contains only the native call and
must not panic or retain destructors across a PostgreSQL nonlocal error jump.
Caller-provided Rust callbacks need their own appropriate error/panic boundary.
An original-address getter merely obtains the address and needs no backend error
guard; a callable adapter remains guarded. Callers must still obey
backend-thread, pointer, ownership, and callback requirements. Read pgrx's
[safety contract](../SAFETY.md) and
[FFI boundary](../pgrx-pg-sys/src/submodules/ffi.rs) before extending this layer.

### Compiling and linking the artifact

`compile_native_support` uses the inspected Clang, language/target/ABI settings,
and original header. It rejects source/object/archive path aliasing, stages
fresh outputs in owned directories, requires new nonempty regular artifacts,
archives the object, and then publishes it. A failed compiler or archiver must
not reuse an old object as success. Object/archive publication is not a single
transaction; an error can occur after the object was replaced.

Preprocess the generated translation unit first with the original inspected
arguments into an owned staged `.i` file. Compile that already-preprocessed input
with `-x cpp-output`, so backend options cannot change header branches such as
`__PIC__` or `__PIE__`. Forced includes and macro definitions participate in the
first phase only. The native backend suppresses unused-argument diagnostics for
those already-consumed preprocessing flags with `-Qunused-arguments`.

Native backend arguments append `-fno-lto`, `-fvisibility=hidden`,
`-ffunction-sections`, and `-fdata-sections`, plus `-fPIC` outside Windows targets.
Verified MSVC objects retain one `oldnames` dependent-library directive for the
CRT's legacy POSIX aliases, even when no runtime mode was recorded. This native
linkage directive does not select a CRT or alter preprocessing definitions.
Disabling LTO keeps ordinary machine code independent of LLVM
release; section flags let the linker discard unused PostgreSQL entry points.
Position-independent-code settings can affect compiler predefines; freezing the
original preprocessing result preserves the selected C header semantics while
still producing a shared-library-compatible object. The extra bounded compiler
invocation is per native artifact, rather than per macro.
Archive format follows the target: Unix-style `.a` or MSVC `.lib`. Archiver
executable spelling follows the host running Clang, including Unix-host MSVC
cross-compilation. This format/tool selection is not evidence of a working
Windows PostgreSQL backend.

Only the active Cargo PostgreSQL major gets a native archive linked. Release
generation may still emit Rust snapshots for all configured supported versions.
Bindgen static-inline wrappers are emitted unconditionally. When `cshim` is
enabled, compile it with the wrappers, macro primitives and metadata in one
translation unit:

```text
 generated pgrx_c_macros_pgNN.c
   #define PGRX_CSHIM_STATIC ".../pgrx-cshim-static-pgNN.c"
   #include ".../pgrx-cshim.c"
       |
       +--> version-specific bindgen static wrapper
                  |
                  +--> original pgNN.h (one include)
   generated macro access/call/address helpers + immutable metadata
       |
       v
 one object --> libpgrx_c_macros_pgNN.a --> extension
```

Some PostgreSQL implementation headers define external functions and have no
include guard. Two objects, or a second header inclusion in one object, can
redefine them. Combining the sources preserves shared external function/global
identity without renaming those definitions. The build returns whether cshim was
integrated. Without cshim, the same active version archive still contains native
inline wrappers, required primitives and metadata. Do not infer integration from
an old file's existence.

## 14. Dependency failures and reports

`MacroDependencyGraph` uses a directed graph whose edge is caller -> dependency.
It contains all final active macro definitions, including retained external and
object-macro context. Ignore formal names, comments, and literals when building
lexical edges. Integer declaration constants have separate user lists; they
are not invented macro nodes. Expansion adds proved synthesized references.

Keep nodes/edges in deterministic name order. Reverse traversal from failure
seeds uses a visited worklist so cycles terminate. Report each impacted caller,
its immediate dependency, and the root reason. A lexical reference is
conservative evidence of dependence, not proof that an invocation expands it.

There are distinct failure behaviors:

1. Preparation refuses a root whose preprocessing dependency closure cannot be
   trusted.
2. A bindgen/Clang integer disagreement seeds reverse failures, even if the
   disagreeing binding is context rather than a selected function macro.
3. A later rendering failure of an already admitted preserved callee propagates
   to callers so no emitted macro references an absent definition.
4. An ordinary initial parse/lowering skip does **not** automatically forbid
   every wrapper. A wrapper can still lower its independently trusted expanded
   body without preserving a call to that unsupported callee.

```text
 C constant witness = Y     bindgen value = X
                \             /
                 \  X != Y   /
                  +----+----+
                       |
                mismatch seed
                       |
          reverse graph traversal
                /             \
        direct users       their callers
             |                  |
       BindingValueMismatch  DependencySkipped
             |                  |
       reason names X/Y    reason names dependency + root reason
```

`SkipReason` carries a code, message, optional token range, and source spans.
Codes cover active-state/provenance failures, unsupported profiles/types,
signature/preprocessing limitations, grammar/literals, pointers/mutation/calls,
unevaluated/type operands, variable access/grouping, budgets/compiler output,
binding disagreement, and dependency failures. Keep codes machine-readable;
do not force clients to classify English message fragments.

A candidate is not a success. `MacroEmission::status` is either emitted source
with its contract or a skip. Shared adapter failures or incoherent compiler
inputs can fail the library generation call as a whole before a new report is
returned. The optional binding-build integration records such failures as
unavailable macro support and publishes its availability classifier. Every
completed generation writes its JSON report. Cargo warnings are an independent
output policy: unset or
`PGRX_MACRO_DEBUG=0` keeps macro-generation diagnostics quiet, while exactly
`PGRX_MACRO_DEBUG=1` emits the summary and each
skip reason with its macro name. Flatten CR/LF in a compiler rejection message
so it remains one Cargo directive; retain the original text in the report.
Track this variable through
`cargo:rerun-if-env-changed` so an incremental build cannot replay diagnostics
from the previous setting. The switch does not change emission, dependency
skips, constant verification, native compilation, or report contents. Explicit
CLI commands retain their own requested output and stderr diagnostics.

## 15. Build integration, files, and public exports

`pgrx-bindgen/src/build.rs` orchestrates the production path:

1. Resolve configurations and active Cargo major.
2. Inspect optional macros with recorded CFLAGS plus the established binding
   target, CPPFLAGS, include settings and environment overrides.
3. Select PostgreSQL function macros, owned variable-dependent object roots and
   every compiler-proven server-owned static inline definition; prepare these
   explicit root kinds in one session. Canonical physical definition ownership
   excludes external headers, including symlink and `..` aliases. Ordinary macro
   inventory/list output remains definitions only.
4. Establish target guards and pgrx opaque integer bridges.
5. Run bindgen with its established CPPFLAGS/include/target invocation; verify
   that the inspected inputs stayed unchanged, then parse its fresh output.
6. Prove and add missing owned integer object constants without overwriting
   existing bindings. Normalize the parsed copy's C foreign/callback ABI to the
   final binding storage's `C-unwind` ABI, then collect the actual `BindingCatalog` and its FFI
   guard path. The existing foreign-function guard rewrite happens afterward.
7. Call `generate_with_bindings`; append shared Rust support and independently
   proved immutable C metadata bridges.
8. Compile native static-inline wrappers, required macro primitives and metadata
   for the active major, integrating optional cshim, then verify inputs again.
9. Apply the existing foreign-function guards and binding rewrites, render the
   macro tree, reports, bindings, and OIDs, and publish fresh artifacts.

If an optional step fails, keep already generated ordinary bindings (or generate
them with the established invocation), write an unavailable macro report and
classifier, and compile the ordinary C shim when enabled. No native macro
linkage is published on that path. Unsupported target families and
`PGRX_C_MACROS=0` skip inspection altogether. Compiler inspection is still
required for a successful macro artifact; fallback never guesses C semantics.

The loaded libclang wrapper permits one live runtime owner in a process. The
build serializes scanner/bindgen inspection around that ownership constraint,
even when version-level generation threads exist. Reuse immutable indexes and
batched compiler probes instead of repeating full header inspection for every
macro.

Typical output layout is:

```text
 OUT_DIR/
   pgNN.rs
   pgNN_oids.rs
   pgNN_macro_report.json
   pgrx-cshim-static-pgNN.c      # bindgen native inline wrappers
   pgrx_c_macros_pgNN.c          # active version primitives + metadata
   pgrx_c_macros_pgNN.o
   libpgrx_c_macros_pgNN.a       # pgrx_c_macros_pgNN.lib on MSVC
   cmacros/
     pgNN/
       mod.rs
       c.rs
       miscadmin.rs
       utils/
         mod.rs
         acl.rs
         memutils.rs
       __pgrx_c_support/
         native_0000.rs         # support fragments; names are ordinal
         shared_0000.rs

 pgrx-pg-sys/src/include/
   pgNN.rs                      # documentation snapshot
   pgNN_oids.rs
   cmacros/pgNN/...              # same organization, release generation
```

Header-relative directories and filenames determine modules. Escape Rust
keywords and invalid filename characters deterministically, handle sanitized
name collisions, and reject duplicate exported macro identifiers. Each public
macro receives `///` documentation with physical header/line provenance and a
fenced `text` block containing its normalized original C definition.

Shared/native Rust files are include fragments, not independent modules. They
preserve lexical scope, private imports, marker paths, and macro definitions.
Split support at complete Rust item boundaries with a 256 KiB soft limit; one
large item can exceed it. Header module files group macro definitions by origin.
Top-level support/module attributes contain the relevant generated-code lint
allowances.

Format macro token trees with the source-preserving formatter first. Rustfmt
can leave repetition-heavy macro transcribers on one line; the custom layout
handles delimiters, punctuation, and generic groups with a preferred width of
100 while retaining spellings and comments. Then stage the tree and run
rustfmt over generated leaves when accessible. A missing executable or precise
rustup missing-component diagnostic permits unformatted output; genuine
formatting/source failures remain errors.

Write changed content stably and remove obsolete files only within the owned
version's generated macro tree. Publish documentation snapshots when
`PGRX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE=1` selects release generation. Normal
builds compile current `OUT_DIR` files; checked-in snapshots are not current
platform inputs. Documentation compatibility is a separate validation pass.
Snapshots retain generated adapter modules and typedef exports; only target
`compile_error!` items are excluded under docsrs. Native interfaces can be
type-checked without PostgreSQL or Clang, but calling their C symbols still
requires a matching archive. Documentation layouts remain release-target facts.

`pgrx-pg-sys/src/include.rs` exposes the `cmacros` selector in
`src/include/cmacros/mod.rs`, which includes the selected `OUT_DIR` tree or
documentation snapshot. `#[macro_export]` defines crate-root macro identifiers;
header modules reexport those identifiers, their parent modules collect them, and
`pgrx/src/lib.rs` uses `pub use pg_sys::cmacros::*`. Thus callers can use
`pgrx::MACRO_NAME!` without caring which header module owns its definition.

Production binding paths start at `$crate::__pgrx_c_bindings`, a public hidden
namespace that reexports only the selected generated binding module. Collect
relative names from that namespace before reconciling declarations, including
aliases, records, fields, callbacks, constants and functions; explicit `crate`
and standard-library paths keep their original targets. Record this root in
`BindingCatalog::namespace`: C-name lookups prepend it, storage lookups retain
complete Rust paths, and imported integer bridges reconcile their exact path
before the matching root-relative path. Callback tags omit this synthetic root
from their public names. Detect top-level OID rewrites at this root rather than
assuming an empty path. This ensures a
handwritten helper or compatibility reexport at the crate root cannot replace
the symbol whose type and value the generator proved. Standalone library
consumers choose the namespace that actually owns their binding declarations.

Cargo invalidation tracks consumed file contents, include/resource directories,
compiler lookup paths, relevant environment values, and PostgreSQL/configuration
inputs. A directory overlapping the output tree cannot be tracked naively as
an input: generation would invalidate itself. Reject unsupported overlapping
search cases or apply the verified ownership rules. There is no permission to
reuse facts solely because the major-version number is unchanged.

There is no persistent disk cache of semantic analysis results. Cargo avoids
rerunning an unchanged build script; sessions reuse their immutable phase facts,
indexes, and batched observations. Stable writes compare final formatted bytes
on a rerun. Track an absent optional path through its nearest existing parent
so a missing file does not make every build dirty and its later creation is
still detected.

Unavailable inspection paths produce an explicit unavailable report and a
compilable macro index whose classifier answers every availability query as absent.
Every normal artifact also includes the classifier, even when every candidate was
skipped or the successful macros take no value operands. Operand adapters are
retained only when successful macros need them; availability needs no C ABI facts.
The discovery library's ability to inspect some targets does not imply that the
current runtime supports their ABI. The runtime gate checks the integer table,
floating and pointer representations, byte order, and a recognized Rust
architecture/OS guard. ARM EABI soft/hard-float and PowerPC64 ELFv1/ELFv2
come from protected compiler witnesses, with matching Rust `target_abi` guards;
integer width agreement cannot establish those procedure-call conventions.
Unmodeled explicit ABI switches remain refused. It admits LP64, LLP64, and ILP32
with either plain-char signedness and byte order. The compiler proves `size_t` and pointer-difference
rank separately; equal widths do not identify C types. Build-script cfg values
select these runtime identities for the active PostgreSQL version.

Rust's `core::ffi::c_char` remains symbolic in the binding catalog because C
char flags do not change the Rust alias. A verified one-byte, all-bit-patterns
valid storage bridge carries its bytes while the inspected C char marker supplies
signedness and promotion. Other incompatible representations still fail proof.

When `pg_config` lacks historical CFLAGS, the current header/target invocation
defines the macro profile, and the audit report records `cflags_recorded: false`.
Real subprocess errors and malformed flags are errors, not absent metadata.
Windows recorded flags use Microsoft quote/backslash rules and a bounded MSVC
option translation; Unix flags use shell quoting. Ordinary bindgen retains its
established CPPFLAGS invocation independently of this optional inspection.

### Complete cross-target artifacts

`PGRX_PG_SYS_EXTRA_TARGET_INFO_PATH` exports a bundle for the active freshly
compiled major. `PGRX_TARGET_INFO_PATH_PGnn` imports that complete bundle;
`PGRX_PG_SYS_EXTRA_OUTPUT_PATH` remains the separate raw-binding-file export.
The manifest inventories raw bindings, the generated macro/support tree, the
compiler report, bindgen wrapper source, native source and target archive. It
records member SHA-256 hashes/lengths, PostgreSQL major, exact Cargo Rust target,
generator version and cshim feature. The report's canonical compiler target must
agree with the expected Rust target too.
Manifest format 2 includes the uniform inline-call interface. Reject older
bundles before publication and regenerate them with the current generator.

Import owns bounded validated bytes before parsing or publishing native linkage.
Missing, corrupt, extra, escaping or mixed-domain members are errors; no raw-only,
checked-in snapshot or handwritten fallback supplies omitted macro/metadata
operations. Bundle-root/internal symlinks are refused; configured ancestor
aliases are resolved. Export stages a complete tree and replaces only an intact
same-domain owned bundle or an empty directory, preserving unrelated data.

Hashes detect corruption, not malicious producers. Generated Rust and native
archives are trusted build code; use a matching pgrx release/revision and target
configuration. Import requires the target PostgreSQL configuration/linker but
does not rerun Clang header inspection. See the
[cross-compilation guide](../docs/src/extension/build/cross-compile.md) for commands
and domain requirements; transport validation is not a claim of cross-target
backend execution.

## 16. A complete generation algorithm

The following pseudocode describes the dependency ordering. Failure handling in
the preceding sections is part of the algorithm, not an optional polish pass.

```text
generate(version, extra_arguments):
    installation = resolve_like_cargo_pgrx(version)
    if complete_target_bundle_is_configured(version):
        raw, macros, native = validate_bundle_for_exact_target_major_features()
        publish_with_existing_binding_guards(raw, macros)
        link_verified_archive_if_active(native)
        return
    arguments = recorded_arguments_then_overrides(installation, extra_arguments)
    frontend = inspect_matching_driver_and_libclang(wrapper(version), arguments)
    macros = final_active_postgres_function_macro_names(frontend)
    objects = owned_variable_dependent_object_roots(frontend)
    inlines = owned_compiler_proven_static_inline_names(frontend)
    session = prepare_with_inline_functions(frontend, macros, objects, inlines)
    names = union_of_prepared_root_names_with_active_macro_precedence(session)

    target_guard_and_integer_bridges = validate_runtime_profile(frontend)
    fresh_rust = run_bindgen_using_existing_cppflags_includes_target_environment(installation)
    verify_session_inputs_again(session)
    binding_ast = parse_fresh_bindings(fresh_rust)
    constants = prove_missing_owned_integer_objects(frontend, binding_ast)
    append_only_missing_constants(fresh_rust, binding_ast, constants)
    normalize_c_to_c_unwind_abi(binding_ast)
    bindings = collect_actual_rust_binding_storage(binding_ast)
    bindings.ffi_boundary = postgres_guard_path

    requests = plan_demands(session, names, bindings)
    adapters = build_validated_adapters_in_dependency_order(requests, bindings)
    capabilities = bindings_with_registered_adapters(bindings, adapters)
    emissions = emit_and_propagate_required_failures(session, names, capabilities)

    successful_names = names_with_emitted_status(emissions)
    retained = plan_demands(session, successful_names, bindings)
    if retained != requests:
        adapters = build_validated_adapters_in_dependency_order(retained, bindings)
    support = render_shared_adapters_and_argument_classifiers(adapters, emissions)

    metadata = prove_original_immutable_c_metadata(frontend, bindings)
    support.rust += metadata.rust
    support.c_source += metadata.c_source
    if active_major(version):
        compile_fresh_archive_with_inline_wrappers_and_optional_cshim(frontend, support)
    verify_session_inputs_again(session)
    files = organize_by_header_provenance(support.rust, emissions)
    final_rust = apply_existing_guards_and_binding_rewrites(fresh_rust)
    record = report(frontend, emissions)
    publish_verified_native_linkage_if_present(support)
    format_stage_and_publish(files, final_rust, record)
    export_complete_target_bundle_if_requested(fresh_rust, files, record, native)
    track_cargo_inputs(session.inputs)
```

The real build also has the optional unavailable paths described above,
per-macro refusals, coherent bundle imports and stable writes. The core
library can inspect an arbitrary C wrapper, analyze selected roots, or
emit standalone macros without pgrx integration. `emit_support_with_bindings`
must return an error if native C support is required; callers that need it use
`generate_with_bindings` or `emit_support_artifact_with_bindings` and compile
the returned artifact under the same profile.

## 17. Walkthrough: a casted arithmetic macro

Consider this definition under the selected supported C profile:

```c
#define F(x) ((uint32_t)((x) + 1))
```

Discovery records `x`, the replacement tokens, and the defining header span.
Prepared expansion keeps `x` as a symbolic hole. Parsing produces:

```text
 Cast uint32_t
       |
      Add
     /   \
 Group   integer literal 1
   |          |
 formal x    concrete C int
```

Analysis assigns the addition a symbolic common type after integer promotion.
The cast target resolves through C declarations to its exact unsigned integer
identity. Fresh bindings can preserve the `uint32_t` Rust alias if available
and verified. At invocation, operation traits select the common type for the
tagged C identity of `x`, perform C addition under the inspected overflow policy,
and then apply the cast's conversion. The public result wraps the evaluated
unsigned value.

This cannot be implemented generally as `T: Add + Into<u32>`. Rust addition does
not impose C promotions, Rust `Into` does not implement truncating C casts, and
one successful `u32` sample says nothing about other admitted C integer ranks.
The generic capability implementation must encode the C rules explicitly.

For an unparenthesized argument occurrence or replacement, the analysis would
also require the corresponding atomic or explicit-boundary contract. The cast
does not grant permission to ignore those substitution rules.

## 18. Resource bounds and complexity

These limits describe the current implementation and are useful defaults for a
reimplementation. Exceeding a limit must produce a recognizable refusal, not
truncate the source into an apparently successful macro.

| Operation | Current bound |
| --- | ---: |
| Production compiler/archiver child duration | 60 seconds |
| Production captured output limit | 16 MiB per stream |
| Static inline candidates | 16,384 |
| Original inline definition text / total retained text | 64 KiB / 16 MiB |
| Bitfield candidates / source / isolation batches | 16,384 / 4 MiB / 128 |
| Builtin source / typed diagnostics | 4 MiB / 256 |
| Builtin LLVM instructions / batches | 128 per witness / 5 |
| Selected roots per default expansion batch | 8,192 |
| Instrumented expansion source | 1 MiB |
| Expanded preprocessor output | 8 MiB |
| Tokens per expanded macro | 8,192 |
| Total expanded batch tokens | 262,144 |
| Dependency-inspection tokens | 1,048,576 |
| Distinct dependencies per root | 4,096 |
| Parser replacement tokens | 4,096 |
| Parser depth | 64 |
| Closed-paste preprocessing passes | 8 |
| Argument normalization formals | 64 |
| Argument classifier token budget | 64 |
| One emitted macro source | 1 MiB |
| Field adapter source | 16 MiB |
| Callback source / structural depth | 16 MiB / 64 |
| Enum adapter source | 16 MiB |
| Function and address adapter source | 16 MiB per family, Rust + C |
| Structural/alias/offset walks | 64 layers or path components |
| Statement capture guard | 64 locals, 4,096 stringified argument bytes |
| Support fragment split threshold | 256 KiB, at complete items |
| Target bundle member / total bytes | 128 MiB / 512 MiB |
| Target bundle manifest / entries / path depth | 4 MiB / 8,192 / 64 components |
| Ordinary C/Rust oracle child duration / output | 30 seconds / 8 MiB |
| Shipping integration child duration / output | 300 seconds / 16 MiB |

Header preprocessing is a substantial fixed cost. Batch expansion, constant
probes, and declaration collection so that cost does not multiply by the number
of macros. The parser and direct tree matching are bounded by their token/arena
sizes. Dependency impact traversal is linear in the visited graph with a
visited set; sorted indexes add their lookup/ordering costs. Demand planning
uses a finite worklist and caches structural compatibility instead of repeatedly
scanning every declaration for every field use.

Do not claim the entire pipeline is linear. Conservative generic families can
retain many records, callbacks, and field capabilities, and compiler subprocess
costs dominate small batches. Source limits and input checks bound the work;
benchmarks should measure real supported profiles before further optimization.
Demand pruning controls emitted support size without letting observed callers
narrow the correctness contract.

## 19. Testing a reimplementation

Use original C definitions and the selected compiler as the oracle. Compile C
and generated Rust separately, run both on defined-domain inputs, and compare:

- Result values and C type identity, including equal-width different ranks.
- Promotions, casts, mixed signedness, division, shifts, and overflow policies.
- Number of argument evaluations, lazy branches, sequencing, and mutations.
- Places, pointer qualifiers, packed/volatile fields, and array behavior.
- Partial aggregate initialization and enum validity without invalid Rust loads.
- Function ABI, original addresses, callbacks, and FFI guard behavior.
- `sizeof`, alignment, `offsetof`, and source zero/null-constant boundaries.
- Substitution, nested macro contexts, local capture, hygiene, and caller returns.
- Rejected invocations, malformed compiler output, stale inputs, and budgets.

Compile-fail tests are essential. A macro that computes a correct number for a
valid call can still be unsound if its traits admit invalid pointer identities,
qualifier loss, ambiguous C ranks, or storage conversions. Native link tests
must prove shared function/global identity and consumption of an ordinary
non-LTO archive, not just successful compilation of two source files.

`tests/support/` builds isolated C/Rust oracle programs. `tests/fixtures/` stores
their definitions; fixture Rust files are test consumers, not macro ground
truth. Normal crate tests need Clang/libclang and exercise pure/native programs,
without using a PostgreSQL database:

```sh
cargo test -p pgrx-c-macros
cargo test -p pgrx-bindgen --lib
```

Installed-header comparisons run in ordinary tests against all configured
supported versions, or the exact `PG_VER` selected by CI. An explicit selection
must resolve an installation; unselected runs without configuration report
omitted coverage. Unsupported runtime profiles must prove an exact macro
refusal before omitting C/Rust comparisons. The historically named
`postgres_handports` target uses test-only original-header roots for migrated
primitives under those same installation-selection rules. Full generated-binding
and release tests remain separate explicit tests:

```sh
cargo test -p pgrx-c-macros --test postgres_emission
cargo test -p pgrx-c-macros --test postgres_handports
cargo test -p pgrx-bindgen --test macro_build -- --ignored
cargo test -p pgrx-bindgen --test shipped_macros -- --ignored
cargo test -p pgrx --test c_macros --no-default-features --features pg18,cshim
cargo test -p pgrx-pg-sys --lib --no-default-features --features pg18,cshim
```

Documentation snapshots require their own release/docs.rs validation. Their
existence is not a substitute for current-platform generation. Tests demonstrate
specific properties; neither successful emission nor a finite oracle sample
proves every possible invocation correct. The type and safety arguments must
stand independently.

## 20. Extending or rebuilding the implementation

Implement the system in dependency order:

1. Build owned discovery, active-state provenance, compiler agreement, and input
   invalidation before accepting translation output.
2. Establish the target/rank model and runtime scalar rules with C comparison
   and compile-fail tests.
3. Add bounded symbolic expansion, constant and zero witnesses, and a complete
   parser for the first admitted grammar.
4. Add symbolic type analysis and context-sensitive argument substitution.
5. Reconcile fresh Rust binding storage, then add places and independently
   validated record/field/offset operations.
6. Add callback/function/enum identities and native primitives with complete ABI
   checks and error-boundary tests.
7. Add demand planning/pruning, readable delegation, deterministic reports,
   publication, and rebuild tracking without relaxing earlier proofs.

For any new syntax, specify its evaluation, C conversion, place, and safety
contracts first. Add an IR representation, analysis rules, runtime capability,
binding/native proof where needed, lowering in every required context, and
independent C/rejection tests. A syntactic parser extension alone does not
establish support. A failed witness remains a skip until its proof can be
generalized; do not add a macro-name exception.

### Source map

| Area | Implementation |
| --- | --- |
| Installation and CLI | [`src/postgres.rs`](src/postgres.rs), [`src/main.rs`](src/main.rs). |
| Discovery and source spans | [`src/lib.rs`](src/lib.rs). |
| Owned C model | [`src/model.rs`](src/model.rs). |
| Compiler/profile/input agreement | [`src/frontend.rs`](src/frontend.rs), [`src/frontend/`](src/frontend/). |
| Expansion and retained symbols | [`src/expansion.rs`](src/expansion.rs), [`constants.rs`](src/expansion/constants.rs), [`paste.rs`](src/expansion/paste.rs). |
| Session and graph | [`src/session.rs`](src/session.rs), [`src/dependencies.rs`](src/dependencies.rs). |
| Grammar and symbolic analysis | [`src/syntax.rs`](src/syntax.rs), [`statements.rs`](src/syntax/statements.rs), [`src/analysis.rs`](src/analysis.rs). |
| Binding representations | [`src/bindings.rs`](src/bindings.rs), [`src/emit/types.rs`](src/emit/types.rs). |
| Batch lowering and support order | [`src/emit.rs`](src/emit.rs), [`demands.rs`](src/emit/demands.rs). |
| Arguments, expressions, statements | [`arguments.rs`](src/emit/arguments.rs), [`typed.rs`](src/emit/typed.rs), [`statements.rs`](src/emit/statements.rs). |
| Native/nominal capabilities | [`fields.rs`](src/emit/fields.rs), [`bitfields.rs`](src/emit/bitfields.rs), [`functions.rs`](src/emit/functions.rs), [`addresses.rs`](src/emit/addresses.rs), [`callbacks.rs`](src/emit/callbacks.rs), [`enumerations.rs`](src/emit/enumerations.rs). |
| Readable delegation and layout | [`src/delegation.rs`](src/delegation.rs), [`src/formatting.rs`](src/formatting.rs). |
| Runtime target gate/bridges | [`src/support_generation.rs`](src/support_generation.rs). |
| Rust semantic runtime | [`pgrx-pg-sys/src/c_macros/`](../pgrx-pg-sys/src/c_macros/). |
| Production binding collector | [`binding_symbols.rs`](../pgrx-bindgen/src/build/binding_symbols.rs). |
| Build/publication/native linkage | [`build.rs`](../pgrx-bindgen/src/build.rs), [`macro_files.rs`](../pgrx-bindgen/src/build/macro_files.rs), [`macro_support.rs`](../pgrx-bindgen/src/build/macro_support.rs). |
| Immutable metadata and target bundles | [`metadata_support.rs`](../pgrx-bindgen/src/build/metadata_support.rs), [`target_artifacts.rs`](../pgrx-bindgen/src/build/target_artifacts.rs). |
| Runtime export | [`pgrx-pg-sys/src/lib.rs`](../pgrx-pg-sys/src/lib.rs), [`pgrx/src/lib.rs`](../pgrx/src/lib.rs). |
| Oracle infrastructure | [`tests/support/`](tests/support/), [`tests/fixtures/`](tests/fixtures/), [`pgrx-bindgen/tests/`](../pgrx-bindgen/tests/). |

For commands, user-facing invocation details, and the library's public surface,
also read the [crate README](README.md) and item documentation. The algorithms
and phase contracts above explain why those APIs are ordered as they are.

## Optional integration and stable identities

Ordinary bindgen generation uses pgrx's existing CPPFLAGS, target includes, and
bindgen environment arguments independently of macro generation. The macro
frontend additionally observes the installation's recorded CFLAGS, and checks
its C declarations against that fresh binding catalog. Rust type storage is
never repaired to make an incompatible C profile appear supported. Failed
inspection or native compilation publishes an unavailable report and classifier,
with no macro definitions or native linkage. `PGRX_C_MACROS=0` skips this optional
work; `PGRX_MACRO_DEBUG=1` enables its Cargo diagnostics. Cross builds require
target PostgreSQL metadata, rather than importing the host's recorded CFLAGS.

Input content fingerprints determine when compiler facts expire. Generated
helper names instead hash semantic target representations, callable signatures,
C overflow policy and layout facts. They do not hash PATH, HOME, working
directories, compiler installation paths or OS deployment versions. Field
markers use the original member name, retaining polymorphic projection across
records without renumbering unrelated fields.

The CLI embeds packaged copies of the canonical wrapper headers. A default
wrapper is published atomically under PGRX_HOME in a directory keyed by its
contents, so its path remains valid after a PostgresConfig is dropped. Explicit
headers bypass materialization. Tests require the packaged wrappers to match
the binding generator's canonical files byte for byte.

Input verification records each file's canonical target separately from its
SHA-256 content identity, so even a same-content symlink retarget invalidates
the inspection. Each verification pass opens and hashes every distinct canonical
input, sharing bytes among aliases only within that pass. Supported Linux/macOS
generator hosts use ring's assembly-backed SHA-256 (built with a host C compiler);
other hosts use SHA2. Backend equivalence tests preserve the report's digest
format, and no mtime-only or cross-pass cache can certify unchanged input bytes.
