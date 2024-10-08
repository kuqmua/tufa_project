mod util;
use util::*;

#[test]
fn rust_decimal() -> TestResult {
    test_default_generated_schema::<rust_decimal1::Decimal>("rust_decimal")
}

#[test]
fn bigdecimal04() -> TestResult {
    test_default_generated_schema::<bigdecimal04::BigDecimal>("bigdecimal04")
}
