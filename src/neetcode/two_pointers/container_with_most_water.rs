/*
You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container contains the most water.

Return the maximum amount of water a container can store.

Notice that you may not slant the container.

Example 1:

Input: height = [1,8,6,2,5,4,8,3,7]
Output: 49
Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
Example 2:

Input: height = [1,1]
Output: 1
*/

fn max_area(height: Vec<i32>) -> i32 {
    // brute force O(n^2) soln
    /*
    let mut max_area = 0;
    for (i, &h1) in height.iter().enumerate() {
        for (j, &h2) in height.iter().enumerate() {
            let curr_area = (j - i) as i32 * std::cmp::min(h1, h2);
            max_area = std::cmp::max(max_area, curr_area);
        }
    }
    max_area
    */

    // two-pointer
    // maximizing area - l all the way at the left, and r all the way at the right
    let (mut l, mut r) = (0, height.len() - 1);
    let mut max_area = 0;

    while l < r {
        let curr_area = (r - l) as i32 * std::cmp::min(height[r], height[l]);
        max_area = std::cmp::max(max_area, curr_area);

        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    max_area
}
