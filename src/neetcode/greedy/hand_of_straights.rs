/*
Alice has some number of cards and she wants to rearrange the cards into groups so that each group is of size groupSize, and consists of groupSize consecutive cards.

Given an integer array hand where hand[i] is the value written on the ith card and an integer groupSize, return true if she can rearrange the cards, or false otherwise.

Example 1:

Input: hand = [1,2,3,6,2,3,4,7,8], groupSize = 3
Output: true
Explanation: Alice's hand can be rearranged as [1,2,3],[2,3,4],[6,7,8]
Example 2:

Input: hand = [1,2,3,4,5], groupSize = 4
Output: false
Explanation: Alice's hand can not be rearranged into groups of 4.

*/

fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
    if hand.len() as i32 % group_size != 0 {
        return false;
    }

    hand.sort();

    while !hand.is_empty() {
        let mut cards_in_hand = 0;
        let mut prev = -1;
        for idx in (0..hand.len()).rev() {
            let curr = hand[idx];
            println!("{hand:?}, {curr}, {prev}");
            if curr == prev {
                continue;
            } else if prev != -1 && curr != prev - 1 {
                // not continuous
                return false;
            }
            hand.remove(idx);
            cards_in_hand += 1;
            prev = curr;

            if cards_in_hand == group_size {
                break;
            }
        }
        if cards_in_hand != group_size {
            // unable to build a straight
            return false;
        }
    }
    true
}
