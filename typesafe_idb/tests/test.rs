#![feature(type_alias_impl_trait)]

#[test]
fn expect_compile_errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
