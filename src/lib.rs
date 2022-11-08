wit_bindgen_rust::export!("maxprofit.wit");
struct Maxprofit;
use crate::maxprofit::Profitable;

impl maxprofit::Maxprofit for Maxprofit {
    fn maxprofit(prices: Vec<f64>) -> Vec<Profitable> {
        if prices.len() < 2 {
            return vec![Profitable {
                buy: -1.0,
                sell: -1.0,
                profit: -1.0,
            }];
        }

        let mut result: Profitable = Profitable {
            buy: prices[0],
            sell: 0.0,
            profit: 0.0,
        };
        let mut max_profit = 0.0;

        for i in 0..prices.len() {
            if result.buy > prices[i] {
                result.buy = prices[i];
            } else if prices[i] - result.buy > max_profit {
                max_profit = prices[i] - result.buy;
                result.sell = prices[i];
                result.profit = max_profit;
            }
        }

        if result.buy > result.sell {
            return vec![Profitable {
                buy: -2.0,
                sell: -2.0,
                profit: -2.0,
            }];
        }

        return vec![result];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
