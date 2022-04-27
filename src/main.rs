use crate::trade_aggregate_metrics::TradeAggregateMetrics;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod aggregate_metrics;
mod trade_aggregate_metrics;

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MainResult<()> {
    let now = std::time::Instant::now();

    let mut io_error_option: Option<std::io::Error> = None;

    let mut trade_dto_parse_error_option: Option<serde_json::Error> = None;

    iterate_stdin_lines()
        .map(|r| match r {
            Ok(line) => Some(line),
            Err(err) => {
                io_error_option = Some(err);
                None
            }
        })
        .take_while(|line_option| line_option.is_some())
        .map(|line_option| line_option.unwrap())
        .skip_while(|line| line.starts_with("BEGIN"))
        .take_while(|line| !line.starts_with("END"))
        .map(|line| {
            let trade_dto_parse_result = serde_json::from_str::<TradeDto>(&line);
            match trade_dto_parse_result {
                Ok(trade_dto) => Some(trade_dto),
                Err(err) => {
                    trade_dto_parse_error_option = Some(err);
                    None
                }
            }
        })
        .take_while(|trade_dto_option| trade_dto_option.is_some())
        .map(|trade_dto_option| trade_dto_option.unwrap())
        .fold(
            HashMap::<u32, TradeAggregateMetrics>::new(),
            |mut map, trade_dto| {
                map.entry(trade_dto.market)
                    .or_insert(TradeAggregateMetrics::new(trade_dto.market))
                    .add_trade(trade_dto.price, trade_dto.volume, trade_dto.is_buy);
                map
            },
        )
        .into_values()
        .map(|trade_aggregate_metrics| {
            let dto = TradeAggregateMetricsDto::from(&trade_aggregate_metrics);
            let dto_json_str = serde_json::to_string(&dto).unwrap();
            println!("{dto_json_str}");
        })
        .count();

    let elapsed = now.elapsed();

    if let Some(io_error) = io_error_option {
        return Err(Box::new(io_error));
    }

    if let Some(trade_dto_json_parse_error) = trade_dto_parse_error_option {
        return Err(Box::new(trade_dto_json_parse_error));
    }

    println!("");
    println!("DONE in {elapsed:.2?}!");

    Ok(())
}

fn iterate_stdin_lines() -> impl Iterator<Item = std::result::Result<String, std::io::Error>> {
    (0..)
        .map(|_| {
            let mut line = String::new();
            let io_result = std::io::stdin().read_line(&mut line);
            io_result.and_then(|_| Ok(line))
        })
        .take_while(|r| r.is_ok() && r.as_ref().unwrap().len() > 0)
}

/// Trade data to receive from input.
#[derive(Deserialize)]
struct TradeDto {
    #[allow(dead_code)]
    id: u32,
    market: u32,
    price: f64,
    volume: f64,
    is_buy: bool,
}

/// Trade aggregate metrics data to pass to output.
#[derive(Serialize)]
#[allow(dead_code)]
struct TradeAggregateMetricsDto {
    market: u32,
    total_volume: f64,
    mean_volume: f64,
    mean_price: f64,
    volume_weighted_average_price: f64,
    percentage_buy: f64,
}

impl TradeAggregateMetricsDto {
    fn from(trade_aggregate_metrics: &TradeAggregateMetrics) -> Self {
        Self {
            market: trade_aggregate_metrics.get_market(),
            total_volume: trade_aggregate_metrics.get_volume_sum(),
            mean_volume: trade_aggregate_metrics.get_volume_mean(),
            mean_price: trade_aggregate_metrics.get_price_mean(),
            volume_weighted_average_price: trade_aggregate_metrics.get_price_volume_weighted_mean(),
            percentage_buy: trade_aggregate_metrics.get_buy_order_percentage(),
        }
    }
}
