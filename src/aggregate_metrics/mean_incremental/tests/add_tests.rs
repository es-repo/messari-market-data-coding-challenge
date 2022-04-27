use super::super::*;
use pretty_assertions::assert_eq;

struct Args {
    x: f64,
    count: u32,
}

#[test]
fn test() {
    test_case(
        MeanIncremental { value: 0.0 },
        Args { x: 1.0, count: 1 },
        MeanIncremental { value: 1.0 },
        "add 1",
    );

    test_case(
        MeanIncremental { value: 1.0 },
        Args { x: 2.0, count: 2 },
        MeanIncremental { value: 1.5 },
        "add 2",
    );

    test_case(
        MeanIncremental { value: 1.5 },
        Args { x: 3.0, count: 3 },
        MeanIncremental { value: 2.0 },
        "add 3",
    );

    test_case(
        MeanIncremental { value: 2.0 },
        Args { x: 4.0, count: 4 },
        MeanIncremental { value: 2.5 },
        "add 4",
    );

    test_case(
        MeanIncremental { value: 2.5 },
        Args { x: 0.0, count: 5 },
        MeanIncremental { value: 2.0 },
        "add 0",
    );

    test_case(
        MeanIncremental { value: 2.0 },
        Args { x: -1.0, count: 6 },
        MeanIncremental { value: 1.5 },
        "add -1",
    );
}

fn test_case(mut init: MeanIncremental, args: Args, expected: MeanIncremental, descr: &str) {
    init.add(args.x, args.count);
    let actual = init;
    assert_eq!(expected, actual, "{}", descr);
}
