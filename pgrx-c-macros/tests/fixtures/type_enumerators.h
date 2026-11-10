/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

/*
 * Macros that paste a fixed prefix onto a type operand to name an enumerator, as
 * PostgreSQL's node macros paste T_, with other names so that nothing can key on those.
 */

#ifndef PGRX_TYPE_ENUMERATORS_H
#define PGRX_TYPE_ENUMERATORS_H

extern void ExceptionalCondition(const char *conditionName, const char *fileName, int lineNumber)
	__attribute__((noreturn));

#ifndef USE_ASSERT_CHECKING
#define Assert(condition) ((void)1)
#else
#define Assert(condition) \
	do { \
		if (!(condition)) \
			ExceptionalCondition(#condition, __FILE__, __LINE__); \
	} while (0)
#endif

typedef enum ShapeKind
{
	KIND_Invalid = 0,
	KIND_Shape = 2,
	KIND_Circle,
	KIND_Square,
	KIND_Segment,
	KIND_Twin,
	KIND_TwinAlias,
	KIND_Masked,
	KIND_MaskedAlias,
	KIND_Unbound,
	KIND_Count
} ShapeKind;

/* A second enum, which the bindings spell as constants instead of a Rust enum. */
typedef enum Paint
{
	PAINT_Circle = 10,
	PAINT_Square = 20
} Paint;

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
} Square;

/* An incomplete type names an enumerator too. */
typedef struct Segment Segment;

/* A structure type without an enumerator. */
typedef struct Plain
{
	int			value;
} Plain;

/* Two C names of one type, each with an enumerator. Rust sees one type. */
typedef struct Twin
{
	ShapeKind	kind;
} Twin;
typedef Twin TwinAlias;

/*
 * A macro hides this type's enumerator, so C would expand the pasted name. The alias's
 * enumerator must not stand in for it.
 */
typedef struct Masked
{
	ShapeKind	kind;
} Masked;
typedef Masked MaskedAlias;
#define KIND_Masked KIND_Circle

/* A type the bindings leave out. KIND_Invalid and KIND_Count name no type at all. */
typedef struct Unbound
{
	ShapeKind	kind;
} Unbound;

typedef struct ShapeList
{
	void	   *first;
} ShapeList;

extern Shape *newShape(unsigned long size, ShapeKind kind);

#define shapeKind(shape) (((const Shape *) (shape))->kind)
#define KindOf(_type_) (KIND_##_type_)
#define BareKind(_type_) KIND_##_type_
#define PaintOf(_type_) (PAINT_##_type_)
#define IsShape(shape, _type_) (shapeKind(shape) == KIND_##_type_)
#define makeShape(_type_) ((_type_ *) newShape(sizeof(_type_), KIND_##_type_))

#ifdef USE_ASSERT_CHECKING
static inline Shape *
castShapeImpl(ShapeKind kind, void *shape)
{
	Assert(shape == 0 || shapeKind(shape) == kind);
	return (Shape *) shape;
}
#define castShape(_type_, shape) ((_type_ *) castShapeImpl(KIND_##_type_, shape))
#else
#define castShape(_type_, shape) ((_type_ *) (shape))
#endif
#define firstShape(_type_, list) castShape(_type_, (list)->first)

/* The paste can sit behind a helper macro, and makes a bare sizeof operand a type. */
#define CppConcat(x, y) x##y
#define KindThrough(_type_) (CppConcat(KIND_, _type_))
#define KindAndSize(_type_) (sizeof(_type_) + KIND_##_type_)

/*
 * Pastes that stay refused: onto the end of an operand, one whose result the expansion
 * drops, one that names no enumerator, one whose operand is also a value, and one that
 * names a member.
 */
#define DROP(x)
#define SuffixOf(_type_) (_type_##_KIND)
#define ErasedKind(_type_) (DROP(KIND_##_type_) 1)
#define UnknownKind(_type_) (NOPE_##_type_)
#define KindAndValue(value) (KIND_##value + (value))
#define KindMember(shape, _type_) ((shape)->kind_##_type_)

#endif
