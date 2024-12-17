use super::IS_EXAMPLE;

pub fn is_example() -> bool {
    *IS_EXAMPLE.read().unwrap()
}
