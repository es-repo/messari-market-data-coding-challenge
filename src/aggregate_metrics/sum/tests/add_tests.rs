use super::super::*;
use pretty_assertions::assert_eq;

struct Args {
    x: f64,
}

#[test]
fn test() {
    test_case(
        Sum { value: 0.0 },
        Args { x: 1.0 },
        Sum { value: 1.0 },
        "add 1",
    );

    test_case(
        Sum { value: 1.0 },
        Args { x: 2.0 },
        Sum { value: 3.0 },
        "add 2",
    );

    test_case(
        Sum { value: 3.0 },
        Args { x: 3.0 },
        Sum { value: 6.0 },
        "add 3",
    );

    test_case(
        Sum { value: 6.0 },
        Args { x: 0.0 },
        Sum { value: 6.0 },
        "add 0",
    );

    test_case(
        Sum { value: 6.0 },
        Args { x: -1.0 },
        Sum { value: 5.0 },
        "add -1",
    );
}

fn test_case(mut init: Sum, args: Args, expected: Sum, descr: &str) {
    init.add(args.x);
    let actual = init;
    assert_eq!(expected, actual, "{}", descr);
}
