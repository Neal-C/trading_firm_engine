mod matching_engine;
use matching_engine::orderbook::{Order, BidOrAsk, OrderBook};
use matching_engine::engine::{MatchingEngine};
use crate::matching_engine::engine::TradingPair;

fn main() {

    let mut matching_engine = MatchingEngine::new();
    let pair = TradingPair::new(String::from("BTC"), String::from("USD"));
    matching_engine.add_new_market(pair.clone());

    let buy_order3 = Order::new(BidOrAsk::Bid, 6.5);
    let eth_pair_market = TradingPair::new("ETH".to_string(), "BTC".to_string());

    matching_engine.place_limit_order(pair, 10.000, buy_order3).unwrap();
    // matching_engine.place_limit_order(eth_pair_market, 10.000, buy_order3).unwrap();

    println!("{:?}", matching_engine);
    // println!("{:?}", orderbook);
}
