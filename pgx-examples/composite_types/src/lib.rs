use pgx::*;

pg_module_magic!();

#[allow(dead_code)]
#[derive(Debug)]
struct MyType {
    name: Option<String>,
    age: Option<i32>,
}

impl FromDatum for MyType {
    unsafe fn from_datum(composite: pg_sys::Datum, is_null: bool) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            let tuple = PgHeapTuple::from_composite_datum(composite);
            Some(Self {
                name: tuple
                    .get_by_name::<String>("name")
                    .expect("failed to get attribute: name"),
                age: tuple
                    .get_by_name::<i32>("age")
                    .expect("failed to get attribute: age"),
            })
        }
    }
}

#[pg_extern]
unsafe fn debug_my_type(
    _my_type: AnyElement,
    fcinfo: pg_sys::FunctionCallInfo,
) -> Option<AnyElement> {
    let composite = pg_getarg_datum(fcinfo, 0).unwrap();
    let mut composite = PgHeapTuple::from_composite_datum(composite);

    composite
        .set_by_name("age", 100)
        .expect("no such attribute");
    composite
        .set_by_name("name", "Someone Else")
        .expect("no such attribute");

    let composite = composite.into_composite_datum().unwrap();
    AnyElement::from_datum(composite, false)
}

#[pg_extern]
fn hello_composite_types() -> &'static str {
    "Hello, composite_types"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_composite_types() {
        assert_eq!("Hello, composite_types", crate::hello_composite_types());
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
