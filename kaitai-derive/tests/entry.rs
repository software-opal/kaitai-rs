#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/enum/01-render-enum.rs");
}
