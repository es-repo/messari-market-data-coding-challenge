use super::super::*;
use pretty_assertions::assert_eq;

#[derive(Clone, Copy)]
struct Args {
    price: f64,
    volume: f64,
    is_buy: bool,
}

#[test]
fn test() {
    let args_1 = Args {
        price: 10.5,
        volume: 1.5,
        is_buy: false,
    };

    let expected_1 = TradeAggregateMetrics {
        market: 1,
        trade_count: Count::new(1),
        volume_sum: Sum::new(args_1.volume),
        volume_mean: MeanIncremental::new(args_1.volume),
        price_mean: MeanIncremental::new(args_1.price),
        price_volume_weighted_mean: WeightedMeanIncremental::new(args_1.price),
        buy_order_percentage: Percentage::new(0),
    };

    let init_1 = TradeAggregateMetrics::new(1);

    test_case(init_1, args_1, &expected_1, "test case 1");

    let args_2 = Args {
        price: 20.2,
        volume: 5.1,
        is_buy: true,
    };
    let init_2: TradeAggregateMetrics = TradeAggregateMetrics { ..expected_1 };

    let expected_2 = TradeAggregateMetrics {
        market: 1,
        trade_count: Count::new(2),
        volume_sum: Sum::new(args_1.volume + args_2.volume),
        volume_mean: MeanIncremental::new((args_1.volume + args_2.volume) / 2.0),
        price_mean: MeanIncremental::new((args_1.price + args_2.price) / 2.0),
        price_volume_weighted_mean: WeightedMeanIncremental::new(
            (args_1.price * args_1.volume + args_2.price * args_2.volume)
                / (args_1.volume + args_2.volume),
        ),
        buy_order_percentage: Percentage::new(1),
    };

    test_case(init_2, args_2, &expected_2, "test case 2");

    let args_3 = Args {
        price: 4.1,
        volume: 3.0,
        is_buy: true,
    };
    let init_3: TradeAggregateMetrics = TradeAggregateMetrics { ..expected_2 };

    let expected_3 = TradeAggregateMetrics {
        market: 1,
        trade_count: Count::new(3),
        volume_sum: Sum::new(args_1.volume + args_2.volume + args_3.volume),
        volume_mean: MeanIncremental::new((args_1.volume + args_2.volume + args_3.volume) / 3.0),
        price_mean: MeanIncremental::new((args_1.price + args_2.price + args_3.price) / 3.0),
        price_volume_weighted_mean: WeightedMeanIncremental::new(
            (args_1.price * args_1.volume
                + args_2.price * args_2.volume
                + args_3.price * args_3.volume)
                / (args_1.volume + args_2.volume + args_3.volume),
        ),
        buy_order_percentage: Percentage::new(2),
    };

    test_case(init_3, args_3, &expected_3, "test case 3");
}

fn test_case(
    mut init: TradeAggregateMetrics,
    args: Args,
    expected: &TradeAggregateMetrics,
    descr: &str,
) {
    init.add_trade(args.price, args.volume, args.is_buy);
    let actual = init;
    assert_eq!(*expected, actual, "{}", descr);
}
