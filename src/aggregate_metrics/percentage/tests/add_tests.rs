use super::super::*;
use pretty_assertions::assert_eq;

#[test]
fn test() {
    test_case(
        Percentage { count_of: 0 },
        Percentage { count_of: 1 },
        "add once",
    );

    test_case(
        Percentage { count_of: 1 },
        Percentage { count_of: 2 },
        "add twice",
    );

    test_case(
        Percentage { count_of: 2 },
        Percentage { count_of: 3 },
        "add thrice",
    );
}

fn test_case(mut init: Percentage, expected: Percentage, descr: &str) {
    init.add();
    let actual = init;
    assert_eq!(expected, actual, "{}", descr);
}
