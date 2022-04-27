use super::super::*;
use pretty_assertions::assert_eq;

#[test]
fn test() {
    test_case(
        TradeAggregateMetrics {
            trade_count: Count::new(1),
            ..TradeAggregateMetrics::new(1)
        },
        0.0,
        "test case 1",
    );

    test_case(
        TradeAggregateMetrics {
            trade_count: Count::new(2),
            buy_order_percentage: Percentage::new(1),
            ..TradeAggregateMetrics::new(1)
        },
        0.5,
        "test case 2",
    );

    test_case(
        TradeAggregateMetrics {
            trade_count: Count::new(10),
            buy_order_percentage: Percentage::new(2),
            ..TradeAggregateMetrics::new(1)
        },
        0.2,
        "test case 3",
    );
}

fn test_case(init: TradeAggregateMetrics, expected: f64, descr: &str) {
    let actual = init.get_buy_order_percentage();
    assert_eq!(expected, actual, "{}", descr);
}
