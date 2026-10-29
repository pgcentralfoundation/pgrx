//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef struct FieldItem { int length; int elements[4]; } FieldItem;
typedef struct FieldState {
    FieldItem *left;
    FieldItem *right;
    int index;
    unsigned int offset;
    _Bool unread;
} FieldState;

#define FIELD_READ(p, member) ((p)->member)
#define FIELD_WRITE(p, member, v) ((p)->member = (v))
#define FIELD_NEXT(cell, state, list, index) \
    ((cell) = ((state).list != (void *)0 && (state).index < (state).list->length) \
        ? &(state).list->elements[(state).index] : (void *)0)
#define FIELD_OFFSET(p, member) ((p)->member == 0 ? (void *)0 : (void *)((char *)(p) + (p)->member))
#define FIELD_SIZE(p, member) (sizeof((p)->member))
#define FIELD_ADDRESS(p, member) (&(p)->member)
#define FIELD_BAD_ROLE(p, member) ((p)->member + (member))
