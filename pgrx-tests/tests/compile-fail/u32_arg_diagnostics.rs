use pgrx::prelude::*;

fn main() {}

// u32 has bad diagnostic UX as an argument type
#[pg_extern]
fn hello_u32(value: u32) -> String {
    format!("Hello {value}")
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_u32() {
        assert_eq!("Hello 10", crate::hello_u32(10));
    }
}

