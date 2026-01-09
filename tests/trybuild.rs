//! Compile-fail tests using trybuild.

#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/*.rs");
}
