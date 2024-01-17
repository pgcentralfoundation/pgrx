use pgrx::prelude::*;
use pgrx::datum::Json;
use serde_json::json;

fn main() {}

// TODO: fix this test by fixing serde impls for `Array<'a, &'a str> -> Json`
#[pg_extern]
fn serde_serialize_array<'dat>(values: Array<'dat, &'dat str>) -> Json {
    Json(json! { { "values": values } })
}

#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::prelude::*;
    use pgrx::Json;
    use serde_json::json;

    // TODO: fix this test by redesigning SPI.
    #[pg_test]
    fn test_serde_serialize_array() -> Result<(), pgrx::spi::Error> {
        let json = Spi::get_one::<Json>(
            "SELECT serde_serialize_array(ARRAY['one', null, 'two', 'three'])",
        )?
        .expect("returned json was null");
        assert_eq!(json.0, json! {{"values": ["one", null, "two", "three"]}});
        Ok(())
    }
}
