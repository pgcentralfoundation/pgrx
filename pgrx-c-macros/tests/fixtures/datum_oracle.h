//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef unsigned int Oid;
typedef unsigned int TransactionId;
typedef __UINTPTR_TYPE__ Datum;
Datum datum_identity(Datum value);
Datum datum_next(Datum value);
unsigned int datum_read(Datum value);
#define DATUM_IDENTITY(value) datum_identity((value))
#define DATUM_NEXT(value) datum_next((value))
#define DATUM_READ(value) datum_read((value))
#define DATUM_POINTER(value) ((void *) (value))
#define DATUM_INTEGER(pointer) ((Datum) (pointer))
