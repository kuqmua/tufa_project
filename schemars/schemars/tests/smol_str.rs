mod util;
use smol_str02::SmolStr;
use util::*;

#[test]
fn smol_str() -> TestResult {
    test_default_generated_schema::<SmolStr>("smol_str")
}
