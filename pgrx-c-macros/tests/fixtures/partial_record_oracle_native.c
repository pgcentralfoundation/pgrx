/* Portions Copyright 2026 PgCentral Foundation, Inc. */
/* Use of this source code is governed by the MIT license. */
PartialRecord partial_record(int value) {
    PartialRecord result;
    result.first = value;
    result.inner.value = value + 1;
    return result;
}
PartialRecord partial_identity(PartialRecord value) { return value; }
int partial_take(PartialRecord value) { return value.first + value.inner.value; }
