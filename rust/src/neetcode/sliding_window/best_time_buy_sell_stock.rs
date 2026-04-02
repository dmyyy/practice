/*

You are given an array prices where prices[i] is the price of a given stock on the ith day.

You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.



Example 1:

Input: prices = [7,1,5,3,6,4]
Output: 5
Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
Example 2:

Input: prices = [7,6,4,3,1]
Output: 0
Explanation: In this case, no transactions are done and the max profit = 0.
*/

use std::cmp::max;

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy_idx = 0;
    let mut sell_idx = 1;
    let mut max_profit = 0;
    while sell_idx < prices.len() {
        if prices[buy_idx] < prices[sell_idx] {
            let profit = prices[sell_idx] - prices[buy_idx];
            max_profit = max(profit, max_profit);
        } else {
            // found new low
            buy_idx = sell_idx;
        }
        sell_idx += 1;
    }
    max_profit
}
