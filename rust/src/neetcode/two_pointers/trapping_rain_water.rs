/*
Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

Example 1:

Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6
Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
Example 2:

Input: height = [4,2,0,3,2,5]
Output: 9
*/

fn trap(height: Vec<i32>) -> i32 {
    // original accepted solution

    // let (mut l, mut r) = (0, height.len() - 1);
    // let (mut max_l, mut max_r) = (height[0], height[height.len() - 1]);
    // let mut trapped_water = 0;
    // while l < r {
    //     let mut maybe_trapped_water = 0;
    //     if max_l <= max_r {
    //         while l < r && max_l <= max_r {
    //             l += 1;
    //             if height[l] > max_l {
    //                 // new max
    //                 max_l = height[l];
    //                 // add + reset trapped water
    //                 trapped_water += maybe_trapped_water;
    //                 maybe_trapped_water = 0;
    //             } else if height[l] == max_l {
    //                 trapped_water += maybe_trapped_water;
    //                 maybe_trapped_water = 0;
    //             } else if height[l] < max_l {
    //                 maybe_trapped_water += max_l - height[l];
    //             }
    //         }
    //     } else {
    //         while l < r && max_r <= max_l {
    //             r -= 1;
    //             if height[r] > max_r {
    //                 // new max
    //                 max_r = height[r];
    //                 // add + reset trapped water
    //                 trapped_water += maybe_trapped_water;
    //                 maybe_trapped_water = 0;
    //             } else if height[r] == max_r {
    //                 trapped_water += maybe_trapped_water;
    //                 maybe_trapped_water = 0;
    //             } else if height[r] < max_r {
    //                 maybe_trapped_water += max_r - height[r];
    //             }
    //         }
    //     }
    // }

    // ai provided solution

    if height.len() < 3 {
        return 0;
    }

    let (mut l, mut r) = (0, height.len() - 1);
    let (mut l_max, mut r_max) = (0, 0);
    let mut water = 0;

    while l < r {
        if height[l] <= height[r] {
            l_max = l_max.max(height[l]);
            water += l_max - height[l];
            l += 1;
        } else {
            r_max = r_max.max(height[r]);
            water += r_max - height[r];
            r -= 1;
        }
    }

    water
}
