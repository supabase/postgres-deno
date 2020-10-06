use pgx::*;

pg_module_magic!();

#[pg_extern]
fn hello_postgres_deno() -> &'static str {
    "Hello, postgres_deno"
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_postgres_deno() {
        assert_eq!("Hello, postgres_deno", crate::hello_postgres_deno());
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
