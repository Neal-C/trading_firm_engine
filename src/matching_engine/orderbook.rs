use std::collections::HashMap;

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    pub fn new() -> Self {
        return Self {
            asks: HashMap::<Price, Limit>::new(),
            bids: HashMap::<Price, Limit>::new()
        }
    }
}

impl OrderBook {
    pub fn add_order(&mut self, price: f64, order: Order){

        let price = Price::new(price);

        match order.bid_or_ask {
            BidOrAsk::Bid => {

                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        limit.add_order(order);
                    },
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            },
            BidOrAsk::Ask => {
                match self.asks.get_mut(&price){
                    Some(limit) => {
                        limit.add_order(order);
                    },
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64
}

impl Price {
    pub fn new(price: f64) -> Self {
        let scalar = 100_000;
        let integral: u64 = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;

        return Self {
            scalar,
            integral,
            fractional
        }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>
}

impl Limit {
    pub fn new(price: Price) -> Self {
        return Self {
            price,
            orders: Vec::<Order>::new(),
        }
    }
}

impl Limit {
    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidOrAsk
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Self {
        return Self {
            bid_or_ask,
            size,
        }
    }
}
