use sari::Error;

#[test]
fn evals_valid_expressions() {
    assert_eq!(sari::eval("(1 + 2) * 3"), Ok(9));
}

#[test]
fn reports_parser_errors() {
    assert_eq!(sari::eval("(1 + 2"), Err(Error::new("expected `)`")));
}

#[test]
fn reports_evaluator_errors() {
    assert_eq!(sari::eval("1 / 0"), Err(Error::new("division by zero")));
}
