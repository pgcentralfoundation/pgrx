/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

/*
 * GNU statement expressions, and the comma expression through a global that compilers
 * without them use instead, as PostgreSQL 15 and 16 define newNode both ways. The names
 * are the fixture's own, so that nothing can key on PostgreSQL's.
 */

#ifndef PGRX_STATEMENT_EXPRESSIONS_H
#define PGRX_STATEMENT_EXPRESSIONS_H

extern void ExceptionalCondition(const char *conditionName, const char *fileName, int lineNumber)
	__attribute__((noreturn));

#ifndef USE_ASSERT_CHECKING
#define CheckMacro(condition) ((void)1)
#else
#define CheckMacro(condition) \
	((void) ((condition) || (ExceptionalCondition(#condition, __FILE__, __LINE__), 0)))
#endif

typedef enum ShapeKind
{
	KIND_Invalid = 0,
	KIND_Shape = 2,
	KIND_Circle,
	KIND_Square,
	KIND_Triangle,
	KIND_Small
} ShapeKind;

typedef struct Shape
{
	ShapeKind	kind;
} Shape;

typedef struct Circle
{
	ShapeKind	kind;
	int			radius;
} Circle;

typedef struct Square
{
	ShapeKind	kind;
	int			side;
	long		area;
} Square;

/* Not a whole number of words, so it takes the other allocator. */
typedef struct Triangle
{
	ShapeKind	kind;
	int			base;
	int			height;
} Triangle;

/* Smaller than a Shape, so a check of its size fails. */
typedef struct Small
{
	char		byte;
} Small;

/* Each allocator returns zeroed storage of at least 16 bytes and records its digit. */
extern void *shape_alloc(unsigned long size);
extern void *shape_alloc_words(unsigned long size);
extern unsigned int shape_trace;
extern unsigned int shape_record(unsigned int digit, unsigned int value);
/* Records the length of its text, then the size. */
extern void shape_note(const char *text, unsigned long size);

#define WORD_MASK (sizeof(long) - 1)
#define WORD_LIMIT 1024
#define WordSized(val, len) \
	( ((len) & WORD_MASK) == 0 && (len) <= WORD_LIMIT && WORD_LIMIT != 0 && (val) == 0 )
#define shapeAlloc(sz) ( WordSized(0, sz) ? shape_alloc_words(sz) : shape_alloc(sz) )

/* The form of a compiler with statement expressions. */
#define blockNewShape(size, tag) \
({	Shape   *_result; \
	CheckMacro((size) >= sizeof(Shape));		/* need the kind, at least */ \
	_result = (Shape *) shapeAlloc(size); \
	_result->kind = (tag); \
	_result; \
})
#define blockMakeShape(_type_) ((_type_ *) blockNewShape(sizeof(_type_), KIND_##_type_))

/* The form of a compiler without them. */
extern Shape *shapeHolder;
#define commaNewShape(size, tag) \
( \
	CheckMacro((size) >= sizeof(Shape)),		/* need the kind, at least */ \
	shapeHolder = (Shape *) shapeAlloc(size), \
	shapeHolder->kind = (tag), \
	shapeHolder \
)
#define commaMakeShape(_type_) ((_type_ *) commaNewShape(sizeof(_type_), KIND_##_type_))

/*
 * A type operand that a diagnostic other than a check stringifies. The pointer type
 * shows that the operand is a type.
 */
#define NOTE_SIZE(_type_) shape_note(#_type_, sizeof(_type_) + sizeof(_type_ *))

/*
 * Statement expressions in general: each operand is evaluated where its local is
 * initialized, statements run in order, and the last expression is the value.
 */
#define BLOCK_MAX(a, b) ({ int _a = (a); int _b = (b); _a > _b ? _a : _b; })
#define BLOCK_ORDER(v) \
	({ shape_record(1, 0); unsigned int _kept = shape_record(2, (v)); shape_record(3, 0); _kept + 1; })
#define BLOCK_IF(pointer) \
	({ int _side = -1; if (pointer) _side = (pointer)->side; else { _side = 0; } _side; })
#define BLOCK_NESTED(v) ({ int _outer = (v); ({ int _inner = _outer + 1; _inner * 2; }) + _outer; })
#define BLOCK_LAZY(c, v) ((c) ? ({ unsigned int _seen = shape_record(4, (v)); _seen; }) : 0u)
/* An unevaluated operand: only the type of the value is used. */
#define BLOCK_SIZE(v) (sizeof(({ long _wide = (v); _wide; })))
/* Inside a statement macro and inside an inline function. */
#define BLOCK_STORE(pointer, v) \
	do { (pointer)->side = ({ int _twice = (v); _twice + _twice; }); } while (0)

static inline int
block_next(int v)
{
	return ({ int _next = v; _next + 1; });
}

/*
 * Statement expressions that stay refused: one without a value, one that returns from
 * its function, a local read before it is initialized, a local that shadows another,
 * a local named outside its block, and the address of a value.
 */
#define BLOCK_VOID(v) ({ int _unused = (v); if (_unused) shape_record(1, 0); })
#define BLOCK_EMPTY_LAST(v) ({ (v); ; })
#define BLOCK_RETURN(v) do { int _early = ({ if (v) return 1; 2; }); (void) _early; } while (0)
#define BLOCK_UNINITIALIZED() ({ int _never; _never; })
#define BLOCK_SHADOW(v) ({ int _twin = (v); ({ int _twin = 1; _twin; }) + _twin; })
#define BLOCK_ESCAPE() (({ int _gone = 1; _gone; }) + _gone)
#define BLOCK_PLACE(v) (&({ int _spot = (v); _spot; }))

#endif
