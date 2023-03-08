use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask
}

#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    fn new() -> Self {
        return Self {
            asks: HashMap::<Price, Limit>::new(),
            bids: HashMap::<Price, Limit>::new()
        }
    }
}

impl OrderBook {
    fn add_order(&mut self, price: f64, order: Order){
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let price = Price::new(price);

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

            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64
}

impl Price {
    fn new(price: f64) -> Self {
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
struct Limit {
    price: Price,
    orders: Vec<Order>
}

impl Limit {
    fn new(price: Price) -> Self {
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
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Self {
        return Self {
            bid_or_ask,
            size,
        }
    }
}

fn main() {
    // let mut limit = Limit::new(65.3);
    let buy_order_from_alice = Order::new(BidOrAsk::Bid,5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);
    // let sell_order = Order::new(BidOrAsk::Ask,5.5);
    // limit.add_order(buy_order);
    // limit.add_order(sell_order);

    let mut orderbook = OrderBook::new();
    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_bob);
    println!("{:?}", orderbook);
}