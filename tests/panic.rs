#[should_panic = "`#[quit::main]` has not been attached to `main`"]
#[test]
fn test_simple() {
    quit::with_code(0);
}
