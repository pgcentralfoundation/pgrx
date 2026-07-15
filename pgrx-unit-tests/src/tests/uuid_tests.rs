//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx::datum::{Uuid, UuidBytes};
use pgrx::prelude::*;

pub const TEST_UUID_V4: UuidBytes = [
    0x12, 0x3e, 0x45, 0x67, 0xe8, 0x9b, 0x12, 0xd3, 0xa4, 0x56, 0x42, 0x66, 0x14, 0x17, 0x40, 0x00,
];

#[pg_extern]
fn accept_uuid(uuid: Uuid) -> Uuid {
    uuid
}

#[pg_extern]
fn return_uuid() -> Uuid {
    Uuid::from_bytes(TEST_UUID_V4)
}

#[pg_extern]
fn display_uuid(uuid: Uuid) -> String {
    format!("{uuid}")
}

// Exercises `Array::<Uuid>::as_slice()`, which is only available because `Uuid: Scalar`.
#[pg_extern]
fn uuid_array_first_via_slice(arr: Array<'_, Uuid>) -> Option<Uuid> {
    arr.as_slice().ok().and_then(|s| s.first().copied())
}

// Round-trips the whole slice back into an array, exercising the `&[Uuid]` layout both ways.
#[pg_extern]
fn uuid_array_roundtrip_via_slice(arr: Array<'_, Uuid>) -> Vec<Uuid> {
    arr.as_slice().unwrap().to_vec()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use core::ptr::NonNull;
    use pgrx::array::{Element, Scalar};
    use pgrx::callconv::DatumPass;
    use pgrx::datum::Uuid;
    use pgrx::layout::PassBy;
    use pgrx::prelude::*;

    // Covers every part of the `Element`/`Scalar` impls for `Uuid` that does not need a live
    // array: the associated OID, the pass-by-ref convention, and the identity `point_from` cast.
    #[pg_test]
    fn test_uuid_scalar_and_element_impls() {
        // Scalar: statically-known OID must be `uuid`.
        assert_eq!(<Uuid as Scalar>::OID, pg_sys::UUIDOID);

        // DatumPass: 16 bytes > size_of::<Datum>() (8), so it must be by-reference.
        assert!(matches!(<Uuid as DatumPass>::PASS, PassBy::Ref));

        // Element::point_from must be a plain identity cast (no header, no endian offset).
        let uuid = Uuid::from_bytes(super::TEST_UUID_V4);
        let ptr = NonNull::from(&uuid).cast::<u8>();
        let pointed = unsafe { <Uuid as Element>::point_from(ptr) };
        assert_eq!(pointed.as_ptr() as usize, ptr.as_ptr() as usize);
        // The default point_from_align4 / borrow_unchecked must observe the same bytes.
        let borrowed = unsafe { <Uuid as Element>::borrow_unchecked::<'_>(ptr) };
        assert_eq!(borrowed, &uuid);
    }

    #[pg_test]
    fn test_uuid_array_first_via_slice() {
        let result = Spi::get_one::<bool>(
            "SELECT uuid_array_first_via_slice(\
                ARRAY['123e4567-e89b-12d3-a456-426614174000'::uuid, \
                      '00000000-0000-0000-0000-000000000001'::uuid]) \
             = '123e4567-e89b-12d3-a456-426614174000'::uuid;",
        );
        assert_eq!(result, Ok(Some(true)));
    }

    #[pg_test]
    fn test_uuid_array_roundtrip_via_slice() {
        let result = Spi::get_one::<bool>(
            "SELECT uuid_array_roundtrip_via_slice(\
                ARRAY['123e4567-e89b-12d3-a456-426614174000'::uuid, \
                      '00000000-0000-0000-0000-000000000001'::uuid]) \
             = ARRAY['123e4567-e89b-12d3-a456-426614174000'::uuid, \
                     '00000000-0000-0000-0000-000000000001'::uuid];",
        );
        assert_eq!(result, Ok(Some(true)));
    }

    #[pg_test]
    fn test_display_uuid() {
        let result = Spi::get_one::<bool>(
            "SELECT display_uuid('123e4567-e89b-12d3-a456-426614174000'::uuid) = '123e4567-e89b-12d3-a456-426614174000';",
        );
        assert_eq!(result, Ok(Some(true)));

        let uuid = Uuid::from_bytes(super::TEST_UUID_V4);
        assert_eq!(format!("{uuid}"), "123e4567-e89b-12d3-a456-426614174000");

        // Lowercase hex formatting
        assert_eq!(format!("{uuid:-x}"), "123e4567-e89b-12d3-a456-426614174000");
        assert_eq!(format!("{uuid:x}"), "123e4567e89b12d3a456426614174000");

        // Uppercase hex formatting
        assert_eq!(format!("{uuid:-X}"), "123E4567-E89B-12D3-A456-426614174000");
        assert_eq!(format!("{uuid:X}"), "123E4567E89B12D3A456426614174000");
    }

    #[pg_test]
    fn test_accept_uuid() {
        let result = Spi::get_one::<bool>(
            "SELECT accept_uuid('123e4567-e89b-12d3-a456-426614174000'::uuid) = '123e4567-e89b-12d3-a456-426614174000'::uuid;",
        );
        assert_eq!(result, Ok(Some(true)));
    }

    #[pg_test]
    fn test_return_uuid() {
        let result = Spi::get_one::<bool>(
            "SELECT return_uuid() = '123e4567-e89b-12d3-a456-426614174000'::uuid;",
        );
        assert_eq!(result, Ok(Some(true)));
    }

    #[pg_test]
    fn test_parse_uuid_v4() {
        let uuid = Spi::get_one::<Uuid>("SELECT '123e4567-e89b-12d3-a456-426614174000'::uuid;");
        assert_eq!(uuid, Ok(Some(Uuid::from_bytes(super::TEST_UUID_V4))));
    }
}
