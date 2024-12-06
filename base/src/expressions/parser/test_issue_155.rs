#![allow(clippy::panic)]

use std::collections::HashMap;

use super::super::parser::stringify::to_string;
use super::super::types::CellReferenceRC;
use super::Parser;

#[test]
fn issue_155_parser() {
    let worksheets = vec!["Sheet1".to_string()];
    let mut parser = Parser::new(worksheets, HashMap::new());

    // Reference cell is Sheet1!A1
    let cell_reference = CellReferenceRC {
        sheet: "Sheet1".to_string(),
        row: 2,
        column: 2,
    };
    let t = parser.parse("A$1:A2", &Some(cell_reference.clone()));
    assert_eq!(to_string(&t, &cell_reference), "A$1:A2");
}

#[test]
fn issue_155_parser_case_2() {
    let worksheets = vec!["Sheet1".to_string()];
    let mut parser = Parser::new(worksheets, HashMap::new());

    // Reference cell is Sheet1!A1
    let cell_reference = CellReferenceRC {
        sheet: "Sheet1".to_string(),
        row: 20,
        column: 20,
    };
    let t = parser.parse("C$1:D2", &Some(cell_reference.clone()));
    assert_eq!(to_string(&t, &cell_reference), "C$1:D2");
}

#[test]
fn issue_155_parser_only_row() {
    let worksheets = vec!["Sheet1".to_string()];
    let mut parser = Parser::new(worksheets, HashMap::new());

    // Reference cell is Sheet1!A1
    let cell_reference = CellReferenceRC {
        sheet: "Sheet1".to_string(),
        row: 20,
        column: 20,
    };
    // This is tricky, I am not sure what to do in these cases
    let t = parser.parse("A$2:B1", &Some(cell_reference.clone()));
    assert_eq!(to_string(&t, &cell_reference), "A1:B$2");
}

#[test]
fn issue_155_parser_only_column() {
    let worksheets = vec!["Sheet1".to_string()];
    let mut parser = Parser::new(worksheets, HashMap::new());

    // Reference cell is Sheet1!A1
    let cell_reference = CellReferenceRC {
        sheet: "Sheet1".to_string(),
        row: 20,
        column: 20,
    };
    // This is tricky, I am not sure what to do in these cases
    let t = parser.parse("D1:$A3", &Some(cell_reference.clone()));
    assert_eq!(to_string(&t, &cell_reference), "$A1:D3");
}