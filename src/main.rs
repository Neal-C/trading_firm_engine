mod matching_engine;
use matching_engine::orderbook::{Order, BidOrAsk, OrderBook};
use matching_engine::engine::{MatchingEngine};
use crate::matching_engine::engine::TradingPair;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

fn main() {

    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 6.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    let mut orderbook = OrderBook::new();
    orderbook.add_limit_order(dec!(4.4), buy_order_from_alice);
    orderbook.add_limit_order(dec!(4.4), buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_limit_order(dec!(20.0), sell_order);

    let mut matching_engine = MatchingEngine::new();

    let pair = TradingPair::new(String::from("BTC"), String::from("USD"));
    matching_engine.add_new_market(pair.clone());

    let buy_order3 = Order::new(BidOrAsk::Bid, 6.5);

    let eth_pair_market = TradingPair::new("ETH".to_string(), "BTC".to_string());

    matching_engine.place_limit_order(pair, dec!(10000), buy_order3).unwrap();

}
