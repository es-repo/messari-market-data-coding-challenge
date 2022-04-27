use super::super::*;
use pretty_assertions::assert_eq;

struct Args {
    value: u32,
}

#[test]
fn test() {
    test_case(
        Args { value: 1 },
        Count { value: 1 },
        "new should be initialized with 0",
    );
}

fn test_case(args: Args, expected: Count, descr: &str) {
    let actual = Count::new(args.value);
    assert_eq!(expected, actual, "{}", descr);
}
