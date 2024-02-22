use pgrx::prelude::*;

fn main() {}

// u32 has bad diagnostic UX as a return type
#[pg_extern]
fn u32_world(value: String) -> u32 {
    value.parse().unwrap()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_u32_world() {
        assert_eq!(10, crate::u32_world("10".to_string()));
    }
}

