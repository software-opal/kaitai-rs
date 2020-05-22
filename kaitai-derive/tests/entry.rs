#[test]
fn enums() {
    let t = trybuild::TestCases::new();
    t.pass("tests/enum/01-render-enum.rs");
}
#[test]
fn types() {
    let t = trybuild::TestCases::new();
    t.pass("tests/types/01-render-simple-type.rs");
}
