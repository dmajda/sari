use sari::{Error, SourcePos, SourceSpan};

#[test]
fn evals_valid_expressions() {
    assert_eq!(sari::eval("(1 + 2) * 3"), Ok(9));
}

#[test]
fn reports_parser_errors() {
    let span = SourceSpan::new(SourcePos::new(6, 1, 7), SourcePos::new(6, 1, 7));
    let error = Error::new(span, "expected `)`");

    assert_eq!(sari::eval("(1 + 2"), Err(error));
}

#[test]
fn reports_evaluator_errors() {
    let span = SourceSpan::new(SourcePos::new(0, 1, 1), SourcePos::new(5, 1, 6));
    let error = Error::new(span, "division by zero");

    assert_eq!(sari::eval("1 / 0"), Err(error));
}
