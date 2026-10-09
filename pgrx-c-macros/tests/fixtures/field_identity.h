//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#include <stddef.h>

typedef struct IdentityOrdinary
{
    unsigned int value;
    const unsigned int frozen;
    unsigned int type;
} IdentityOrdinary;

typedef struct IdentityOther
{
    unsigned char prefix;
    unsigned int value;
} IdentityOther;

typedef struct IdentityPromoted
{
    struct
    {
        const unsigned int frozen;
        unsigned int value;
    };
} IdentityPromoted;

typedef struct IdentityBits
{
    unsigned int value : 5;
    const unsigned int frozen : 3;
} IdentityBits;

typedef struct IdentityKeyword
{
    unsigned int type;
} IdentityKeyword;

/* Intentionally absent from the bindgen allowlist. Its earlier field spelling
 * exercises identities that must survive removal of a rejected macro root. */
typedef struct IdentityDiscarded
{
    unsigned int aa_discarded;
} IdentityDiscarded;

#define IDENTITY_READ(p, member) ((p)->member)
#define IDENTITY_WRITE(p, member, v) ((p)->member = (v))
#define IDENTITY_OFFSET(type, member) offsetof(type, member)
#define IDENTITY_PRIMARY(p) (((IdentityOrdinary *)(p))->value)
#define IDENTITY_OTHER(p) (((IdentityOther *)(p))->value)
#define IDENTITY_PROMOTED(p) (((IdentityPromoted *)(p))->frozen)
#define IDENTITY_BITS_READ(p) (((IdentityBits *)(p))->value)
#define IDENTITY_BITS_WRITE(p, v) (((IdentityBits *)(p))->value = (v))
#define IDENTITY_KEYWORD(p) (((IdentityKeyword *)(p))->type)
#define IDENTITY_SKIPPED(p) (((IdentityDiscarded *)(p))->aa_discarded)
