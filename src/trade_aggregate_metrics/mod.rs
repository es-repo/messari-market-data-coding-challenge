use crate::aggregate_metrics::*;

#[derive(Debug, PartialEq)]
pub struct TradeAggregateMetrics {
    market: u32,
    trade_count: Count,
    volume_sum: Sum,
    volume_mean: MeanIncremental,
    price_mean: MeanIncremental,
    price_volume_weighted_mean: WeightedMeanIncremental,
    buy_order_percentage: Percentage,
}

impl TradeAggregateMetrics {
    pub fn new(market: u32) -> TradeAggregateMetrics {
        let trade_count = Count::new(0);
        let volume_sum = Sum::new(0.0);
        let volume_mean = MeanIncremental::new(0.0);
        let price_mean = MeanIncremental::new(0.0);
        let price_volume_weighted_mean = WeightedMeanIncremental::new(0.0);
        let buy_order_percentage = Percentage::new(0);

        TradeAggregateMetrics {
            trade_count,
            market,
            volume_sum,
            volume_mean,
            price_mean,
            price_volume_weighted_mean,
            buy_order_percentage,
        }
    }

    pub fn add_trade(&mut self, price: f64, volume: f64, is_buy: bool) {
        self.trade_count.add();
        self.volume_sum.add(volume);
        self.volume_mean.add(volume, self.trade_count.get_value());
        self.price_mean.add(price, self.trade_count.get_value());
        self.price_volume_weighted_mean
            .add(price, volume, self.volume_sum.get_value());

        if is_buy {
            self.buy_order_percentage.add()
        }
    }

    pub fn get_market(&self) -> u32 {
        self.market
    }

    pub fn get_volume_sum(&self) -> f64 {
        self.volume_sum.get_value()
    }

    pub fn get_volume_mean(&self) -> f64 {
        self.volume_mean.get_value()
    }

    pub fn get_price_mean(&self) -> f64 {
        self.price_mean.get_value()
    }

    pub fn get_price_volume_weighted_mean(&self) -> f64 {
        self.price_volume_weighted_mean.get_value()
    }

    pub fn get_buy_order_percentage(&self) -> f64 {
        self.buy_order_percentage
            .get_value(self.trade_count.get_value())
    }
}

#[cfg(test)]
mod tests;
