use super::super::*;
use pretty_assertions::assert_eq;

struct Args {
    x: f64,
    w: f64,
    weighted_sum: f64,
}

#[test]
fn test() {
    test_case(
        WeightedMeanIncremental { value: 0.0 },
        Args {
            x: 1.0,
            w: 0.5,
            weighted_sum: 0.5,
        },
        WeightedMeanIncremental { value: 1.0 },
    );

    test_case(
        WeightedMeanIncremental { value: 1.0 },
        Args {
            x: 2.0,
            w: 1.5,
            weighted_sum: 2.0,
        },
        WeightedMeanIncremental { value: 1.75 },
    );

    test_case(
        WeightedMeanIncremental { value: 1.75 },
        Args {
            x: 3.0,
            w: 1.0,
            weighted_sum: 3.0,
        },
        WeightedMeanIncremental {
            value: 2.1666666666666665,
        },
    );

    test_case(
        WeightedMeanIncremental {
            value: 2.1666666666666665,
        },
        Args {
            x: 0.0,
            w: 0.0,
            weighted_sum: 3.0,
        },
        WeightedMeanIncremental {
            value: 2.1666666666666665,
        },
    );

    test_case(
        WeightedMeanIncremental {
            value: 2.1666666666666665,
        },
        Args {
            x: -1.0,
            w: -1.0,
            weighted_sum: 2.0,
        },
        WeightedMeanIncremental { value: 3.75 },
    );
}

fn test_case(mut init: WeightedMeanIncremental, args: Args, expected: WeightedMeanIncremental) {
    init.add(args.x, args.w, args.weighted_sum);
    let actual = init;
    assert_eq!(expected, actual);
}
