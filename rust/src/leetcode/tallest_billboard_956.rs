/*
You are installing a billboard and want it to have the largest height. The billboard will have two steel supports, one on each side. Each steel support must be an equal height.

You are given a collection of rods that can be welded together. For example, if you have rods of lengths 1, 2, and 3, you can weld them together to make a support of length 6.

Return the largest possible height of your billboard installation. If you cannot support the billboard, return 0.

Example 1:

Input: rods = [1,2,3,6]
Output: 6
Explanation: We have two disjoint subsets {1,2,3} and {6}, which have the same sum = 6.
Example 2:

Input: rods = [1,2,3,4,5,6]
Output: 10
Explanation: We have two disjoint subsets {2,3,5} and {4,6}, which have the same sum = 10.
Example 3:

Input: rods = [1,2]
Output: 0
Explanation: The billboard cannot be supported, so we return 0.
*/

// first attempt:
// - didn't handle the case of skipping a rod
// - stop recursing once we find a match
// - possibility space is too large - exploring all permutations (rod1, rod2, index)

/*
fn tallest_billboard(rods: Vec<i32>) -> i32 {
    if rods.len() <= 1 {
        return 0;
    }

    fn tb(rod1: i32, rod2: i32, rods: &[i32], maxes: &mut Vec<i32>) {
        println!("{rod1}, {rod2}, {rods:?}");
        if rod1 != 0 && rod1 == rod2 {
            maxes.push(rod1);
        } else if !rods.is_empty() {
            tb(rod1 + rods[0], rod2, &rods[1..], maxes);
            tb(rod1, rod2 + rods[0], &rods[1..], maxes);
        }
    }

    // consider making maxes a binary heap
    let mut maxes = Vec::new();
    tb(0, 0, rods.as_slice(), &mut maxes);

    // find max in maxes
    maxes.into_iter().max().unwrap_or(0)
}
*/

// TODO: do this correctly 

fn tallest_billboard(rods: Vec<i32>) -> i32 {
    // explore possibility space
    // (diff, idx)
}
