use super::super::*;
use pretty_assertions::assert_eq;

#[test]
fn test() {
    test_case(Count { value: 0 }, Count { value: 1 }, "add once");

    test_case(Count { value: 1 }, Count { value: 2 }, "add twice");

    test_case(Count { value: 2 }, Count { value: 3 }, "add thrice");
}

fn test_case(mut init: Count, expected: Count, descr: &str) {
    init.add();
    let actual = init;
    assert_eq!(expected, actual, "{}", descr);
}
