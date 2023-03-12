use std::collections::HashMap;
use crate::{Order, OrderBook};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;


// Market : BTCUSD
// BTC => Base
// USD => Quote

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> Self {
        return Self {
            base,
            quote
        }
    }

    pub fn to_string(&self) -> String{
       return format!("{}_{}", self.base, self.quote);
    }
}

#[derive(Debug)]
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>
}

impl MatchingEngine {
    pub fn new() -> Self {
        return Self {
            orderbooks: HashMap::<TradingPair,OrderBook>::new()
        }
    }
}

impl MatchingEngine {
    pub fn add_new_market(&mut self, pair: TradingPair){
        println!("opening new orderbook for market {}", pair.to_string());
        self.orderbooks.insert(pair, OrderBook::new());
    }

    pub fn place_limit_order(&mut self, pair: TradingPair, price: Decimal, order: Order) ->
                                                                                        Result<(),
        String> {
        match self.orderbooks.get_mut(&pair){
            Some(orderbook) => {
                println!("placed limit order {:?}", order);
                orderbook.add_limit_order(price, order);
                println!("place limit order at price level {price}");
               return Ok(());
            },
            None => {
                return Err(
                    format!("the orderbook for the given market ({}) does not exist yet", pair
                    .to_string())
                );
            }
        };
    }
}