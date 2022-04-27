use super::super::*;
use pretty_assertions::assert_eq;

struct Args {
    count: u32,
}

#[test]
fn test() {
    test_case(
        Percentage { count_of: 0 },
        Args { count: 1 },
        0.0,
        "test case 1",
    );

    test_case(
        Percentage { count_of: 1 },
        Args { count: 1 },
        1.0,
        "test case 2",
    );

    test_case(
        Percentage { count_of: 50 },
        Args { count: 100 },
        0.5,
        "test case 3",
    );

    test_case(
        Percentage { count_of: 2 },
        Args { count: 10 },
        0.2,
        "test case 4",
    );

    test_case(
        Percentage { count_of: 10 },
        Args { count: 2 },
        5.0,
        "test case 5",
    );
}

fn test_case(init: Percentage, args: Args, expected: f64, descr: &str) {
    let actual = init.get_value(args.count);
    assert_eq!(expected, actual, "{}", descr);
}
